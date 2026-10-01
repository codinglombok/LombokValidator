//! Cross-language conformance runner (ADR-002/015). `LOMBOK_REGEN=1` rewrites
//! expected outputs from THIS reference implementation — review the diff.

use lombokvalidator::*;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vectors/lombokvalidator-vectors-v1.json")
}
fn i64_of(v: &Value) -> i64 {
    match v {
        Value::String(t) => t.parse().unwrap(),
        n => n.as_i64().unwrap(),
    }
}
fn opt_i64(v: &Value) -> Option<i64> {
    if v.is_null() {
        None
    } else {
        Some(i64_of(v))
    }
}
fn opt_f64(v: &Value) -> Option<f64> {
    if v.is_null() {
        None
    } else {
        v.as_f64()
    }
}
fn opt_usize(v: &Value) -> Option<usize> {
    if v.is_null() {
        None
    } else {
        v.as_u64().map(|n| n as usize)
    }
}
fn argjson(a: &ArgValue) -> Value {
    match a {
        ArgValue::Str(s) => json!(s),
        ArgValue::Int(i) => json!(i.to_string()), // i64 as string: JSON numbers lose precision >2^53 in JS
        ArgValue::UInt(u) => json!(u),
        ArgValue::Float(f) => json!(f),
    }
}
fn res(r: ValidationResult) -> Value {
    match r {
        Ok(()) => json!({"out": null}),
        Err(es) => json!({"out": es.iter().map(|e| {
            let mut args = serde_json::Map::new();
            for (k, v) in &e.args { args.insert(k.clone(), argjson(v)); }
            json!({"field": e.field, "code": e.code, "messageId": e.message_id, "args": args})
        }).collect::<Vec<_>>()}),
    }
}

fn eval(c: &Value) -> Value {
    let i = &c["in"];
    let sv = || i.as_str().unwrap();
    match c["fn"].as_str().unwrap() {
        "is_email" => json!({"out": is_email(sv())}),
        "is_url" => json!({"out": is_url(sv())}),
        "is_uuid" => json!({"out": is_uuid(sv())}),
        "is_integer_str" => json!({"out": is_integer_str(sv())}),
        "is_float_str" => json!({"out": is_float_str(sv())}),
        "strip_html" => json!({"out": strip_html(sv())}),
        "trim" => json!({"out": trim(sv())}),
        "to_lowercase" => json!({"out": to_lowercase(sv())}),
        "validate_length" => res(validate_length(
            "f",
            i["value"].as_str().unwrap(),
            opt_usize(&i["min"]),
            opt_usize(&i["max"]),
        )),
        "validate_range_i64" => res(validate_range_i64(
            "f",
            i64_of(&i["value"]),
            opt_i64(&i["min"]),
            opt_i64(&i["max"]),
        )),
        "validate_range_f64" => res(validate_range_f64(
            "f",
            i["value"].as_f64().unwrap(),
            opt_f64(&i["min"]),
            opt_f64(&i["max"]),
        )),
        "validate_positive_i64" => res(validate_positive_i64("f", i64_of(i))),
        "validate_positive_f64" => res(validate_positive_f64("f", i.as_f64().unwrap())),
        "matches_pattern" => {
            match try_matches_pattern(i["value"].as_str().unwrap(), i["pattern"].as_str().unwrap())
            {
                Ok(b) => json!({"out": b}),
                Err(PatternError::TooComplex) => json!({"err": "TooComplex"}),
            }
        }
        "validate_pattern" => res(validate_pattern(
            "f",
            i["value"].as_str().unwrap(),
            i["pattern"].as_str().unwrap(),
        )),
        other => panic!("unknown fn {other}"),
    }
}

#[test]
fn conformance() {
    let doc: Value = serde_json::from_str(&fs::read_to_string(path()).unwrap()).unwrap();
    let regen = std::env::var("LOMBOK_REGEN").is_ok();
    let mut lines = Vec::new();
    let mut failures = Vec::new();
    for (n, c) in doc["cases"].as_array().unwrap().iter().enumerate() {
        let got = eval(c);
        let mut merged = json!({"fn": c["fn"], "in": c["in"]});
        for (k, v) in got.as_object().unwrap() {
            merged[k] = v.clone();
        }
        if !regen && (c.get("out") != got.get("out") || c.get("err") != got.get("err")) {
            failures.push(format!(
                "case {n} {} in={} expected={:?} got={}",
                c["fn"],
                c["in"],
                c.get("out").or(c.get("err")),
                got
            ));
        }
        lines.push(serde_json::to_string(&merged).unwrap());
    }
    if regen {
        fs::write(
            path(),
            format!(
                "{{\"suite\":\"lombokvalidator\",\"version\":1,\"cases\":[\n{}\n]}}\n",
                lines.join(",\n")
            ),
        )
        .unwrap();
        return;
    }
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
