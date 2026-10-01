//! Zero-dependency backtracking regex engine for input validation.
//!
//! Supported: literals, `.`, classes `[abc]` `[^abc]` `[a-z]`, shorthands
//! `\d \w \s`, greedy quantifiers `* + ? {n} {n,} {n,m}`, anchors `^ $`, and
//! top-level alternation `a|b`. **Not** supported: groups, lookaround,
//! backreferences, lazy quantifiers.
//!
//! Safety: matching is bounded by a step budget ([`MAX_STEPS`]). A pattern
//! that exceeds it yields [`PatternError::TooComplex`] instead of hanging, so
//! catastrophic backtracking cannot be used for denial of service even if a
//! pattern is chosen badly. Patterns should still come from trusted code.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// Upper bound on backtracking steps per match attempt.
pub const MAX_STEPS: u64 = 1_000_000;
const MAX_REPEAT: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternError {
    /// The step budget was exhausted (pathological pattern/input pair).
    TooComplex,
}

#[derive(Debug, Clone)]
enum Token {
    Literal(char),
    AnyChar,
    Class {
        negate: bool,
        ranges: Vec<(char, char)>,
    },
    /// `inner{min,max}` (`None` = unbounded).
    Rep(Box<Token>, usize, Option<usize>),
    StartAnchor,
    EndAnchor,
}

fn parse_class(chars: &[char], i: &mut usize) -> Token {
    *i += 1; // '['
    let negate = chars.get(*i) == Some(&'^');
    if negate {
        *i += 1;
    }
    let mut ranges = Vec::new();
    let mut first = true;
    while let Some(&c) = chars.get(*i) {
        if c == ']' && !first {
            *i += 1;
            break;
        }
        first = false;
        let c = if c == '\\' {
            *i += 1;
            chars.get(*i).copied().unwrap_or('\\')
        } else {
            c
        };
        if chars.get(*i + 1) == Some(&'-') && chars.get(*i + 2).is_some_and(|e| *e != ']') {
            let end = chars[*i + 2];
            ranges.push((c, end));
            *i += 3;
        } else {
            ranges.push((c, c));
            *i += 1;
        }
    }
    Token::Class { negate, ranges }
}

/// Parse `{n}`, `{n,}`, `{n,m}` at `chars[*i]` (which is `{`). Returns None
/// (and leaves `i`) if it is not a valid counted quantifier.
fn parse_counted(chars: &[char], i: &mut usize) -> Option<(usize, Option<usize>)> {
    let mut j = *i + 1;
    let num = |j: &mut usize| -> Option<usize> {
        let st = *j;
        while chars.get(*j).is_some_and(|c| c.is_ascii_digit()) {
            *j += 1;
        }
        if *j == st || *j - st > 4 {
            return None;
        }
        chars[st..*j].iter().collect::<String>().parse().ok()
    };
    let min = num(&mut j)?;
    let max = if chars.get(j) == Some(&',') {
        j += 1;
        if chars.get(j) == Some(&'}') {
            None
        } else {
            Some(num(&mut j)?)
        }
    } else {
        Some(min)
    };
    if chars.get(j) != Some(&'}') {
        return None;
    }
    if max.is_some_and(|m| m < min) {
        return None;
    }
    *i = j + 1;
    Some((min.min(MAX_REPEAT), max.map(|m| m.min(MAX_REPEAT))))
}

fn tokenize(pattern: &str) -> Vec<Token> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let base = match chars[i] {
            '^' => {
                tokens.push(Token::StartAnchor);
                i += 1;
                continue;
            }
            '$' => {
                tokens.push(Token::EndAnchor);
                i += 1;
                continue;
            }
            '.' => {
                i += 1;
                Token::AnyChar
            }
            '[' => parse_class(&chars, &mut i),
            '\\' => {
                i += 1;
                let esc = chars.get(i).copied().unwrap_or('\\');
                i += 1;
                match esc {
                    'd' => Token::Class {
                        negate: false,
                        ranges: alloc::vec![('0', '9')],
                    },
                    'w' => Token::Class {
                        negate: false,
                        ranges: alloc::vec![('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')],
                    },
                    's' => Token::Class {
                        negate: false,
                        ranges: alloc::vec![(' ', ' '), ('\t', '\t'), ('\n', '\n'), ('\r', '\r')],
                    },
                    other => Token::Literal(other),
                }
            }
            other => {
                i += 1;
                Token::Literal(other)
            }
        };
        match chars.get(i) {
            Some('*') => {
                i += 1;
                tokens.push(Token::Rep(Box::new(base), 0, None));
            }
            Some('+') => {
                i += 1;
                tokens.push(Token::Rep(Box::new(base), 1, None));
            }
            Some('?') => {
                i += 1;
                tokens.push(Token::Rep(Box::new(base), 0, Some(1)));
            }
            Some('{') => match parse_counted(&chars, &mut i) {
                Some((min, max)) => tokens.push(Token::Rep(Box::new(base), min, max)),
                None => tokens.push(base), // literal '{' handled next loop
            },
            _ => tokens.push(base),
        }
    }
    tokens
}

fn matches_single(tok: &Token, c: char) -> bool {
    match tok {
        Token::Literal(l) => *l == c,
        Token::AnyChar => true,
        Token::Class { negate, ranges } => {
            ranges.iter().any(|(lo, hi)| c >= *lo && c <= *hi) != *negate
        }
        _ => false,
    }
}

type Steps = u64;

fn match_here(
    tokens: &[Token],
    ti: usize,
    input: &[char],
    ci: usize,
    steps: &mut Steps,
) -> Result<bool, PatternError> {
    *steps += 1;
    if *steps > MAX_STEPS {
        return Err(PatternError::TooComplex);
    }
    if ti == tokens.len() {
        return Ok(ci == input.len());
    }
    match &tokens[ti] {
        Token::StartAnchor => {
            if ci == 0 {
                match_here(tokens, ti + 1, input, ci, steps)
            } else {
                Ok(false)
            }
        }
        Token::EndAnchor => {
            if ci == input.len() {
                match_here(tokens, ti + 1, input, ci, steps)
            } else {
                Ok(false)
            }
        }
        Token::Rep(inner, min, max) => {
            let limit = max.unwrap_or(usize::MAX);
            let mut run = 0usize;
            while run < limit && ci + run < input.len() && matches_single(inner, input[ci + run]) {
                run += 1;
            }
            if run < *min {
                return Ok(false);
            }
            let mut n = run as isize;
            while n >= *min as isize {
                if match_here(tokens, ti + 1, input, ci + n as usize, steps)? {
                    return Ok(true);
                }
                n -= 1;
            }
            Ok(false)
        }
        simple => {
            if ci < input.len() && matches_single(simple, input[ci]) {
                match_here(tokens, ti + 1, input, ci + 1, steps)
            } else {
                Ok(false)
            }
        }
    }
}

/// Full-string match; `Err(TooComplex)` if the step budget is exhausted.
pub fn try_matches_pattern(input: &str, pattern: &str) -> Result<bool, PatternError> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut steps: Steps = 0;
    for alt in pattern.split('|') {
        let tokens = tokenize(alt);
        if match_here(&tokens, 0, &input_chars, 0, &mut steps)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Full-string match. Returns `false` if the step budget is exhausted
/// (fail-closed); use [`try_matches_pattern`] to distinguish.
pub fn matches_pattern(input: &str, pattern: &str) -> bool {
    try_matches_pattern(input, pattern).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_dot_classes() {
        assert!(matches_pattern("hello", "hello"));
        assert!(!matches_pattern("hello", "world"));
        assert!(matches_pattern("abc123", "[a-z]+[0-9]*"));
        assert!(!matches_pattern("123abc", "[a-z]+[0-9]*"));
        assert!(matches_pattern("a.c", "a.c"));
        assert!(matches_pattern("]", "[]]"));
    }

    #[test]
    fn shorthands_anchors_alternation_optional() {
        assert!(matches_pattern("12345", "\\d+"));
        assert!(!matches_pattern("12a45", "\\d+"));
        assert!(matches_pattern("hello", "^hello$"));
        assert!(matches_pattern("dog", "cat|dog"));
        assert!(!matches_pattern("fish", "cat|dog"));
        assert!(matches_pattern("colour", "colou?r") && matches_pattern("color", "colou?r"));
        assert!(matches_pattern("abc", "[^0-9]+") && !matches_pattern("ab1", "[^0-9]+"));
    }

    #[test]
    fn counted_quantifiers() {
        assert!(matches_pattern("2026", "\\d{4}"));
        assert!(!matches_pattern("202", "\\d{4}"));
        assert!(matches_pattern("123", "\\d{2,}"));
        assert!(matches_pattern("12", "\\d{1,3}"));
        assert!(!matches_pattern("1234", "\\d{1,3}"));
        assert!(matches_pattern("AB-1234", "[A-Z]{2}-\\d{4}"));
        assert!(matches_pattern("a{", "a\\{") || matches_pattern("a{", "a{"));
    }

    #[test]
    fn catastrophic_backtracking_is_bounded() {
        let input = "a".repeat(3000) + "b";
        let pat = "a*a*a*a*a*a*c"; // never matches; would be O(n^6)
        assert_eq!(
            try_matches_pattern(&input, pat),
            Err(PatternError::TooComplex)
        );
        assert!(!matches_pattern(&input, pat)); // fail-closed
    }

    #[test]
    fn normal_inputs_stay_far_below_budget() {
        let long = "x".repeat(100_000);
        assert_eq!(try_matches_pattern(&long, "x+"), Ok(true));
    }

    #[test]
    fn malformed_patterns_do_not_panic() {
        for p in [
            "[", "[a-", "\\", "a{", "a{1", "a{2,1}", "a{99999}", "{}", "|||", "[]", "[^]", "a**",
            "*", "+a",
        ] {
            let _ = try_matches_pattern("aaa", p);
            let _ = try_matches_pattern("", p);
        }
    }
}
