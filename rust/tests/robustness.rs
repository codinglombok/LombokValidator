//! Dependency-free pseudo-fuzz: arbitrary patterns/inputs never panic and
//! always terminate within the step budget.

use lombokvalidator::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn pick<'a>(&mut self, xs: &[&'a str]) -> &'a str {
        xs[(self.next() % xs.len() as u64) as usize]
    }
}

const PAT: &[&str] = &[
    "a", "b", ".", "*", "+", "?", "{", "}", "{2}", "{1,3}", "[", "]", "^", "$", "|", "\\", "\\d",
    "\\w", "-", "[a-z]", "[^a]", ",",
];
const TXT: &[&str] = &[
    "a", "b", "ab", "aaaa", "é", "日", "@", ".", " ", "1", "-", "<", ">", "\0", "http://", "x@y.co",
];

fn make(rng: &mut Rng, set: &[&str], max: u64) -> String {
    (0..rng.next() % max).map(|_| rng.pick(set)).collect()
}

#[test]
fn random_patterns_and_inputs_never_panic() {
    let mut rng = Rng(0xD1B54A32D192ED03);
    for _ in 0..30_000 {
        let pat = make(&mut rng, PAT, 14);
        let txt = make(&mut rng, TXT, 30);
        let _ = try_matches_pattern(&txt, &pat);
        let _ = validate_pattern("f", &txt, &pat);
        let _ = validate_email("f", &txt);
        let _ = validate_url("f", &txt);
        let _ = validate_uuid("f", &txt);
        let _ = validate_length("f", &txt, Some(2), Some(20));
        let _ = strip_html(&txt);
        let _ = is_float_str(&txt);
        let _ = is_integer_str(&txt);
    }
}

#[test]
fn hostile_sizes_terminate() {
    let big = "a".repeat(200_000);
    let _ = try_matches_pattern(&big, "a*a*a*b");
    let _ = try_matches_pattern(&big, "(a|a)*"); // group syntax unsupported: literal chars, must not hang
    let _ = validate_email("f", &big);
    let _ = strip_html(&"<".repeat(200_000));
    let _ = strip_html(&"<a".repeat(100_000));
}
