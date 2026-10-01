//! Validation errors carry a stable `code` (the contract other systems
//! match against) and a `message_id` for lookup in a LombokLocale
//! catalog — the "code (kontrak) + messageId (terjemahan)" convention
//! from ARCHITECTURE_UTAMA §7.

use alloc::string::String;
use alloc::vec::Vec;
use lomboklocale::ArgValue;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub code: &'static str,
    pub message_id: &'static str,
    /// Arguments for the catalog message pattern, e.g. `min`/`max`/`actual`.
    pub args: Vec<(String, ArgValue)>,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, code: &'static str, message_id: &'static str) -> Self {
        ValidationError {
            field: field.into(),
            code,
            message_id,
            args: Vec::new(),
        }
    }

    pub fn with_arg(mut self, name: impl Into<String>, value: ArgValue) -> Self {
        self.args.push((name.into(), value));
        self
    }

    /// Render a human-readable fallback (English, hardcoded) for contexts
    /// with no catalog loaded yet — useful in tests and quick scripts.
    /// Production code should resolve `message_id` through a loaded
    /// LombokLocale [`lomboklocale::Catalog`] instead.
    pub fn default_message(&self) -> String {
        alloc::format!("{}: validation failed ({})", self.field, self.code)
    }
}

pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Merge several validation results into one, collecting every error
/// instead of stopping at the first (needed for schema/object validation).
pub fn merge_results(results: Vec<ValidationResult>) -> ValidationResult {
    let mut errors = Vec::new();
    for r in results {
        if let Err(mut e) = r {
            errors.append(&mut e);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
