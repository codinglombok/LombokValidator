//! Validation errors carry a stable `code` (the contract other systems
//! match against) and a `message_id` for lookup in a LombokLocale
//! catalog — the "code (kontrak) + messageId (terjemahan)" convention
//! from ARCHITECTURE_UTAMA §7.

use alloc::string::String;
use alloc::vec::Vec;

/// Typed message argument carried by every [`ValidationError`]. This type is
/// part of the cross-language contract (serialized in the shared test
/// vectors: `Str` as JSON string, `Int` as decimal string, `UInt`/`Float` as
/// JSON numbers) — it is deliberately owned by LombokValidator, not
/// LombokLocale, so locale API changes cannot shift the contract.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgValue {
    Str(String),
    Int(i64),
    UInt(u64),
    Float(f64),
}

impl From<&ArgValue> for lomboklocale::Arg {
    /// Convert into the catalog formatter's argument type. Only used at
    /// message-rendering time; never part of the wire contract.
    fn from(v: &ArgValue) -> Self {
        match v {
            ArgValue::Str(s) => lomboklocale::Arg::Str(s.clone()),
            ArgValue::Int(i) => lomboklocale::Arg::from(*i),
            ArgValue::UInt(u) => lomboklocale::Arg::from(*u),
            ArgValue::Float(f) => lomboklocale::Arg::from(*f),
        }
    }
}

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
