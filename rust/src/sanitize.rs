//! Sanitization: trim, lowercase, and a conservative HTML tag stripper.
//!
//! `strip_html` here is a minimal, allocation-only tag remover for
//! sanitizing plain-text fields (e.g. a "display name" input) — it is
//! *not* a substitute for LombokHTML's real sanitizer for untrusted
//! rich-text/markup, which understands scripts, comments, and CDATA.

use alloc::string::String;

pub fn trim(s: &str) -> &str {
    s.trim()
}

pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// Strip anything that looks like an HTML/XML tag (`<...>`), including
/// its contents. Does not decode entities. Angle brackets appearing
/// without a matching close are left as-is (fails safe: nothing after an
/// unclosed `<` is silently dropped).
pub fn strip_html(s: &str) -> String {
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    // A '<' opens a tag only if some '>' follows it. Precomputing the last '>'
    // makes that check O(1) per '<' (was O(n) → quadratic on '<<<<…').
    let last_gt = chars.iter().rposition(|c| *c == '>');
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for (i, &c) in chars.iter().enumerate() {
        if !in_tag && c == '<' {
            if last_gt.is_some_and(|g| g > i) {
                in_tag = true;
            } else {
                out.push(c);
            }
        } else if in_tag && c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_whitespace() {
        assert_eq!(trim("  hello  "), "hello");
    }

    #[test]
    fn lowercases() {
        assert_eq!(to_lowercase("HELLO World"), "hello world");
    }

    #[test]
    fn strips_simple_tags() {
        assert_eq!(strip_html("<b>bold</b> text"), "bold text");
        assert_eq!(strip_html("<script>alert(1)</script>safe"), "alert(1)safe");
    }

    #[test]
    fn leaves_unmatched_angle_bracket() {
        assert_eq!(strip_html("5 < 10"), "5 < 10");
    }

    #[test]
    fn linear_time_on_many_open_brackets() {
        let start = std::time::Instant::now();
        let out = strip_html(&"<".repeat(300_000));
        assert_eq!(out.len(), 300_000);
        assert!(start.elapsed().as_secs() < 2);
    }

    #[test]
    fn strips_attributes() {
        assert_eq!(
            strip_html(r#"<a href="evil.com" onclick="steal()">link</a>"#),
            "link"
        );
    }
}
