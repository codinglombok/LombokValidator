//! Smoke test over the BUILT library (cargo run --release --example smoke):
//! a real end-to-end validation of a signup form, nested object, sanitization,
//! and stable error codes.

use lombokvalidator::*;

struct Address {
    postcode: &'static str,
}
impl Validate for Address {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("postcode", || {
                validate_length("postcode", self.postcode, Some(5), Some(5))
            })
            .validate_all()
    }
}

struct Order {
    email: &'static str,
    qty: i64,
    address: Address,
}
impl Validate for Order {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("email", || validate_email("email", self.email))
            .field("qty", || validate_positive_i64("qty", self.qty))
            .nested("address", &self.address)
            .validate_all()
    }
}

fn main() {
    // 1. Valid form passes.
    let ok = Order {
        email: "buyer@example.co.id",
        qty: 3,
        address: Address { postcode: "83517" },
    };
    assert!(ok.validate().is_ok());

    // 2. Invalid form collects ALL errors with stable codes and nested paths.
    let bad = Order {
        email: "bukan-email",
        qty: 0,
        address: Address { postcode: "12" },
    };
    let errs = bad.validate().unwrap_err();
    let mut codes: Vec<&str> = errs.iter().map(|e| e.code).collect();
    codes.sort_unstable();
    assert_eq!(codes, vec!["INVALID_EMAIL", "NOT_POSITIVE", "TOO_SHORT"]);
    assert!(errs.iter().any(|e| e.field == "address.postcode"));
    println!("errors collected: {:?}", codes);
    for e in &errs {
        println!("  {} — {} (default: {})", e.field, e.code, e.default_message());
    }

    // 3. Sanitization round used by real callers.
    let stripped = sanitize::strip_html("  <b>Wayan</b> ");
    let name = sanitize::trim(&stripped);
    assert_eq!(name, "Wayan");

    // 4. Pattern with fail-closed budget.
    assert!(matches_pattern("AB-1234", "[A-Z]{2}-\\d{4}"));

    println!("SMOKE OK: lombokvalidator 0.1.0 (built library, real example)");
}
