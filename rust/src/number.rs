//! Number validators: range, integer/float shape, positivity.

use crate::error::{ValidationError, ValidationResult};
use lomboklocale::ArgValue;

pub fn validate_range_i64(
    field: &str,
    value: i64,
    min: Option<i64>,
    max: Option<i64>,
) -> ValidationResult {
    if let Some(min) = min {
        if value < min {
            return Err(alloc::vec![ValidationError::new(
                field,
                "BELOW_MIN",
                "validator.number.below_min"
            )
            .with_arg("min", ArgValue::Int(min))
            .with_arg("actual", ArgValue::Int(value))]);
        }
    }
    if let Some(max) = max {
        if value > max {
            return Err(alloc::vec![ValidationError::new(
                field,
                "ABOVE_MAX",
                "validator.number.above_max"
            )
            .with_arg("max", ArgValue::Int(max))
            .with_arg("actual", ArgValue::Int(value))]);
        }
    }
    Ok(())
}

pub fn validate_range_f64(
    field: &str,
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
) -> ValidationResult {
    if let Some(min) = min {
        if value < min {
            return Err(alloc::vec![ValidationError::new(
                field,
                "BELOW_MIN",
                "validator.number.below_min"
            )
            .with_arg("min", ArgValue::Float(min))
            .with_arg("actual", ArgValue::Float(value))]);
        }
    }
    if let Some(max) = max {
        if value > max {
            return Err(alloc::vec![ValidationError::new(
                field,
                "ABOVE_MAX",
                "validator.number.above_max"
            )
            .with_arg("max", ArgValue::Float(max))
            .with_arg("actual", ArgValue::Float(value))]);
        }
    }
    Ok(())
}

pub fn validate_positive_i64(field: &str, value: i64) -> ValidationResult {
    if value > 0 {
        Ok(())
    } else {
        Err(alloc::vec![ValidationError::new(
            field,
            "NOT_POSITIVE",
            "validator.number.not_positive"
        )
        .with_arg("actual", ArgValue::Int(value))])
    }
}

pub fn validate_positive_f64(field: &str, value: f64) -> ValidationResult {
    if value > 0.0 {
        Ok(())
    } else {
        Err(alloc::vec![ValidationError::new(
            field,
            "NOT_POSITIVE",
            "validator.number.not_positive"
        )
        .with_arg("actual", ArgValue::Float(value))])
    }
}

/// True if `s` parses as an integer (optional leading `-`, digits only).
pub fn is_integer_str(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// True if `s` parses as a float (one optional `-`, digits, optional
/// single `.` followed by digits).
pub fn is_float_str(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);
    if s.is_empty() {
        return false;
    }
    let mut seen_dot = false;
    let mut seen_digit = false;
    for c in s.chars() {
        if c == '.' {
            if seen_dot {
                return false;
            }
            seen_dot = true;
        } else if c.is_ascii_digit() {
            seen_digit = true;
        } else {
            return false;
        }
    }
    seen_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_i64() {
        assert!(validate_range_i64("age", 25, Some(0), Some(120)).is_ok());
        assert!(validate_range_i64("age", -1, Some(0), Some(120)).is_err());
        assert!(validate_range_i64("age", 200, Some(0), Some(120)).is_err());
    }

    #[test]
    fn range_f64() {
        assert!(validate_range_f64("score", 7.5, Some(0.0), Some(10.0)).is_ok());
        assert!(validate_range_f64("score", 11.0, Some(0.0), Some(10.0)).is_err());
    }

    #[test]
    fn positive_checks() {
        assert!(validate_positive_i64("qty", 5).is_ok());
        assert!(validate_positive_i64("qty", 0).is_err());
        assert!(validate_positive_i64("qty", -5).is_err());
        assert!(validate_positive_f64("price", 0.01).is_ok());
        assert!(validate_positive_f64("price", -0.01).is_err());
    }

    #[test]
    fn integer_and_float_shape() {
        assert!(is_integer_str("1234"));
        assert!(is_integer_str("-1234"));
        assert!(!is_integer_str("12.34"));
        assert!(!is_integer_str(""));
        assert!(is_float_str("12.34"));
        assert!(is_float_str("-0.5"));
        assert!(is_float_str("5"));
        assert!(!is_float_str("1.2.3"));
        assert!(!is_float_str("abc"));
    }
}
