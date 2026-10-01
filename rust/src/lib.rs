//! LombokValidator — zero-runtime-dependency-*except-LombokLocale*
//! validation core (L1).
//!
//! Implements the scope from `10_LombokValidator.md`: string validation
//! (email/URL/UUID/length/pattern), number validation (range/positive),
//! schema validation (struct-level, nested), trait-based custom
//! validators, and sanitization — with i18n-ready errors resolved
//! through LombokLocale (ADR-009), the one permitted L0 dependency for
//! an L1 repo.
//!
//! `no_std + alloc` by default; `std` feature only toggles LombokLocale's
//! own `std` feature through.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod error;
pub mod number;
pub mod pattern;
pub mod sanitize;
pub mod schema;
pub mod string;

pub use error::{ValidationError, ValidationResult};
pub use number::{
    is_float_str, is_integer_str, validate_positive_f64, validate_positive_i64, validate_range_f64,
    validate_range_i64,
};
pub use pattern::{matches_pattern, try_matches_pattern, PatternError, MAX_STEPS};
pub use sanitize::{strip_html, to_lowercase, trim};
pub use schema::{ClosureValidator, FieldValidator, Schema, Validate};
pub use string::{
    is_email, is_url, is_uuid, validate_email, validate_length, validate_pattern, validate_url,
    validate_uuid,
};

// Re-export so downstream crates don't need a direct LombokLocale
// dependency just to build error-message args.
pub use lomboklocale::ArgValue;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use lomboklocale::{parse_catalog, resolve};

    #[test]
    fn error_resolves_through_lomboklocale_catalog() {
        let catalog = parse_catalog(
            r#"{"validator.string.too_short": "{field} minimal {min} karakter (saat ini {actual})"}"#,
        )
        .unwrap();
        let err = validate_length("username", "ab", Some(4), None).unwrap_err();
        let e = &err[0];
        let mut args: alloc::vec::Vec<(&str, ArgValue)> = e
            .args
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        args.push(("field", ArgValue::Str(e.field.clone())));
        let msg = resolve("id", &catalog, e.code, e.message_id, &args);
        assert_eq!(msg.text, "username minimal 4 karakter (saat ini 2)");
    }
}
