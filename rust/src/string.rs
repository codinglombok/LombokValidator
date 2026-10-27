//! String validators: email, URL, UUID, length/pattern.
//!
//! Email/URL/UUID are hand-rolled structural checks (not full RFC 5322 /
//! RFC 3986 parsers) — sufficient for input validation without pulling in
//! an external dependency.

use crate::error::{ArgValue, ValidationError, ValidationResult};
use crate::pattern::{try_matches_pattern, PatternError};

/// Structural email check with RFC 5321 length limits: `local@domain.tld`;
/// local part 1–64 chars without whitespace/`@`; total ≤ 254; domain labels
/// 1–63 chars of alphanumerics/hyphens, not starting or ending with `-`;
/// at least one `.`; TLD ≥ 2 ASCII letters. Not a full RFC 5322 parser
/// (quoted local parts and IP-literal domains are rejected; internationalized
/// domains must be given in A-label/punycode form).
pub fn is_email(s: &str) -> bool {
    if s.len() > 254 {
        return false;
    }
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    if local.is_empty() || local.len() > 64 || domain.is_empty() {
        return false;
    }
    // RFC 5322 `atext` + `.`; non-ASCII allowed in the local part (RFC 6531 /
    // SMTPUTF8). Quotes, spaces and `( ) , : ; < > @ [ \` are rejected.
    let atext = |c: char| {
        c.is_ascii_alphanumeric()
            || "!#$%&'*+/=?^_`{|}~-.".contains(c)
            || (!c.is_ascii() && !c.is_whitespace() && !c.is_control())
    };
    if !local.chars().all(atext) {
        return false;
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }
    if !domain.contains('.') {
        return false;
    }
    let labels: alloc::vec::Vec<&str> = domain.split('.').collect();
    for l in &labels {
        if l.is_empty()
            || l.len() > 63
            || l.starts_with('-')
            || l.ends_with('-')
            || !l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return false;
        }
    }
    let tld = labels.last().unwrap();
    tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic())
}

/// Structural URL check: `scheme://[userinfo@]host[:port][/path][?query][#fragment]`
/// with a valid scheme (letter, then letters/digits/`+`/`-`/`.`), a non-empty
/// host, and no whitespace, control characters, or `< > \" \\ ^ ` { | }`.
pub fn is_url(s: &str) -> bool {
    // No whitespace/control characters and none of ` < > " \\ ^ ` { | } anywhere.
    if s.chars()
        .any(|c| c.is_whitespace() || c.is_control() || "<>\"\\^`{|}".contains(c))
    {
        return false;
    }
    let Some((scheme, rest)) = s.split_once("://") else {
        return false;
    };
    if scheme.is_empty()
        || !scheme.chars().next().unwrap().is_ascii_alphabetic()
        || !scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
    {
        return false;
    }
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    !host.is_empty() && !host.starts_with(':')
}

/// RFC 4122 UUID (8-4-4-4-12 hex, case-insensitive).
pub fn is_uuid(s: &str) -> bool {
    let groups: alloc::vec::Vec<&str> = s.split('-').collect();
    let expected_lens = [8usize, 4, 4, 4, 12];
    if groups.len() != 5 {
        return false;
    }
    groups
        .iter()
        .zip(expected_lens.iter())
        .all(|(g, len)| g.len() == *len && g.chars().all(|c| c.is_ascii_hexdigit()))
}

/// `min..=max` character-count length check (inclusive). Pass `None` for
/// an unbounded side.
pub fn validate_length(
    field: &str,
    value: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> ValidationResult {
    let len = value.chars().count();
    if let Some(min) = min {
        if len < min {
            return Err(alloc::vec![ValidationError::new(
                field,
                "TOO_SHORT",
                "validator.string.too_short"
            )
            .with_arg("min", ArgValue::UInt(min as u64))
            .with_arg("actual", ArgValue::UInt(len as u64))]);
        }
    }
    if let Some(max) = max {
        if len > max {
            return Err(alloc::vec![ValidationError::new(
                field,
                "TOO_LONG",
                "validator.string.too_long"
            )
            .with_arg("max", ArgValue::UInt(max as u64))
            .with_arg("actual", ArgValue::UInt(len as u64))]);
        }
    }
    Ok(())
}

pub fn validate_email(field: &str, value: &str) -> ValidationResult {
    if is_email(value) {
        Ok(())
    } else {
        Err(alloc::vec![ValidationError::new(
            field,
            "INVALID_EMAIL",
            "validator.string.invalid_email"
        )])
    }
}

pub fn validate_url(field: &str, value: &str) -> ValidationResult {
    if is_url(value) {
        Ok(())
    } else {
        Err(alloc::vec![ValidationError::new(
            field,
            "INVALID_URL",
            "validator.string.invalid_url"
        )])
    }
}

pub fn validate_uuid(field: &str, value: &str) -> ValidationResult {
    if is_uuid(value) {
        Ok(())
    } else {
        Err(alloc::vec![ValidationError::new(
            field,
            "INVALID_UUID",
            "validator.string.invalid_uuid"
        )])
    }
}

pub fn validate_pattern(field: &str, value: &str, pattern: &str) -> ValidationResult {
    match try_matches_pattern(value, pattern) {
        Ok(true) => Ok(()),
        Ok(false) => Err(alloc::vec![ValidationError::new(
            field,
            "PATTERN_MISMATCH",
            "validator.string.pattern_mismatch"
        )]),
        Err(PatternError::TooComplex) => Err(alloc::vec![ValidationError::new(
            field,
            "PATTERN_TOO_COMPLEX",
            "validator.string.pattern_too_complex"
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valid_and_invalid() {
        assert!(is_email("user@example.com"));
        assert!(is_email("first.last@sub.example.co.id"));
        assert!(!is_email("no-at-sign.com"));
        assert!(!is_email("user@"));
        assert!(!is_email("@example.com"));
        assert!(!is_email("user@example"));
        assert!(!is_email("has space@example.com"));
    }

    #[test]
    fn email_local_part_charset() {
        assert!(!is_email("\"quoted\"@example.com"));
        assert!(!is_email("a b@example.com"));
        assert!(!is_email("a(b)@example.com"));
        assert!(!is_email("a<b>@example.com"));
        assert!(!is_email("a[b]@example.com"));
        assert!(is_email("o'brien@example.com"));
        assert!(is_email("a_b-c+d=e@example.com"));
        assert!(is_email("ünï@example.com")); // SMTPUTF8 local part
        assert!(!is_email("a@ünï.com")); // IDN domain must be punycode
        assert!(is_email("a@xn--nni-jma.com"));
    }

    #[test]
    fn url_rejects_unsafe_characters() {
        assert!(!is_url("https://exa mple.com"));
        assert!(!is_url("https://example.com/a b"));
        assert!(!is_url("https://example.com/<x>"));
        assert!(!is_url("https://example.com/\"x"));
        assert!(!is_url("https://user@:80/"));
        assert!(is_url("https://user:pw@host:8080/p?q=%20#f"));
        assert!(is_url("https://例え.jp/パス"));
    }

    #[test]
    fn email_limits_and_label_rules() {
        assert!(!is_email(&alloc::format!("{}@example.com", "a".repeat(65))));
        assert!(is_email(&alloc::format!("{}@example.com", "a".repeat(64))));
        assert!(!is_email(&alloc::format!("a@{}.com", "b".repeat(64))));
        assert!(!is_email(&alloc::format!(
            "a@{}.{}.com",
            "b".repeat(63),
            "c".repeat(200)
        )));
        assert!(!is_email("a@-bad.com"));
        assert!(!is_email("a@bad-.com"));
        assert!(!is_email("a..b@example.com"));
        assert!(!is_email(".a@example.com"));
        assert!(!is_email("a@exam ple.com"));
        assert!(!is_email("a@b..com"));
        assert!(is_email("a+tag@sub-domain.example.co.id"));
    }

    #[test]
    fn pattern_too_complex_has_own_code() {
        let input = "a".repeat(3000) + "b";
        let e = validate_pattern("f", &input, "a*a*a*a*a*a*c").unwrap_err();
        assert_eq!(e[0].code, "PATTERN_TOO_COMPLEX");
    }

    #[test]
    fn url_valid_and_invalid() {
        assert!(is_url("https://example.com"));
        assert!(is_url("https://example.com/path?q=1#frag"));
        assert!(is_url("ftp://files.example.com"));
        assert!(!is_url("not a url"));
        assert!(!is_url("://missing-scheme.com"));
        assert!(!is_url("https://"));
    }

    #[test]
    fn uuid_valid_and_invalid() {
        assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_uuid("550E8400-E29B-41D4-A716-446655440000"));
        assert!(!is_uuid("not-a-uuid"));
        assert!(!is_uuid("550e8400-e29b-41d4-a716"));
    }

    #[test]
    fn length_bounds() {
        assert!(validate_length("username", "abc", Some(2), Some(10)).is_ok());
        assert!(validate_length("username", "a", Some(2), Some(10)).is_err());
        assert!(validate_length("username", "way-too-long-value", Some(2), Some(10)).is_err());
    }

    #[test]
    fn pattern_validation() {
        assert!(validate_pattern("code", "AB1234", "[A-Z]+[0-9]+").is_ok());
        assert!(validate_pattern("code", "1234AB", "[A-Z]+[0-9]+").is_err());
    }

    #[test]
    fn error_carries_code_and_message_id() {
        let err = validate_email("email", "bad").unwrap_err();
        assert_eq!(err[0].code, "INVALID_EMAIL");
        assert_eq!(err[0].message_id, "validator.string.invalid_email");
    }
}
