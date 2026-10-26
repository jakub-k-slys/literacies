//! String to Integer (atoi) as a parser that can say no.
//!
//! Coding Literacy - if zero is a valid answer, it cannot also be the error.
//!
//! Two layers, as in the Python and Scala siblings. `parse_int32_prefix` is the
//! core and returns `Result<i32, PrefixError>`: `Ok(value)` for a number that
//! fit, `Err(NoDigits)` when nothing was read, `Err(OutOfRange { negative })`
//! when the number did not fit. That is the Python sibling's `Parsed`,
//! `NoDigits` and `OutOfRange`, in the shape Rust uses for every fallible
//! operation, so `?` and `match` work on it. `my_atoi` is the adapter the judge
//! calls, and the only place that decides 0 means "nothing here".
//!
//! The grammar is ASCII: the space character `' '` only, one optional sign,
//! digits `b'0'..=b'9'`. It runs over bytes, which is safe for any `&str`:
//! every byte of a multi-byte UTF-8 character is 0x80 or above, so none of them
//! can be mistaken for a space, a sign or a digit.
//!
//! The sign stays in the digits. A negative number accumulates downward
//! (`rev * 10 + (-d)`), so `i32::MIN` is reachable without a wider type, where
//! the Python sibling briefly holds the magnitude 2147483648. Every push goes
//! through `checked_mul` and `checked_add`, and `None` becomes
//! `OutOfRange`, so the overflow check is the boundary check from Reverse
//! Integer with the tie-break done by the type.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixError {
    NoDigits,
    OutOfRange { negative: bool },
}

impl fmt::Display for PrefixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrefixError::NoDigits => write!(f, "no digits"),
            PrefixError::OutOfRange { negative: true } => write!(f, "below i32::MIN"),
            PrefixError::OutOfRange { negative: false } => write!(f, "above i32::MAX"),
        }
    }
}

impl std::error::Error for PrefixError {}

pub fn parse_int32_prefix(s: &str) -> Result<i32, PrefixError> {
    let bytes = s.as_bytes(); // no byte of a multi-byte character is ' ', a sign or a digit
    let mut i = 0;
    while bytes.get(i) == Some(&b' ') {
        i += 1; // the space character only, not trim_start()
    }
    let negative = bytes.get(i) == Some(&b'-');
    if matches!(bytes.get(i), Some(b'+' | b'-')) {
        i += 1;
    }
    let start = i;
    let mut rev: i32 = 0;
    while let Some(&c @ b'0'..=b'9') = bytes.get(i) {
        let d = i32::from(c - b'0');
        let d = if negative { -d } else { d }; // the sign stays in the digits: i32::MIN needs no widening
        let next = rev.checked_mul(10).and_then(|r| r.checked_add(d));
        rev = next.ok_or(PrefixError::OutOfRange { negative })?; // None: the push would leave i32
        i += 1;
    }
    if i == start {
        return Err(PrefixError::NoDigits); // the index never moved past the sign
    }
    Ok(rev)
}

pub fn my_atoi(s: &str) -> i32 {
    match parse_int32_prefix(s) {
        Ok(value) => value,
        Err(PrefixError::NoDigits) => 0, // the one place that decides 0 means "nothing here"
        Err(PrefixError::OutOfRange { negative: true }) => i32::MIN,
        Err(PrefixError::OutOfRange { negative: false }) => i32::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixError::{NoDigits, OutOfRange};
    use super::{my_atoi, parse_int32_prefix};
    use std::num::IntErrorKind;

    const NBSP: &str = "\u{a0}"; // a non-breaking space, kept as an escape on purpose
    const ARABIC_42: &str = "\u{664}\u{662}"; // "٤٢", Arabic-Indic four and two

    #[test]
    fn statement_examples() {
        assert_eq!(my_atoi("42"), 42);
        assert_eq!(my_atoi("   -42"), -42);
        assert_eq!(my_atoi("4193 with words"), 4193);
        assert_eq!(my_atoi("words and 987"), 0);
        assert_eq!(my_atoi("-91283472332"), i32::MIN);
        assert_eq!(my_atoi("0"), 0);
    }

    #[test]
    fn zero_and_no_digits_are_one_answer_from_atoi() {
        for s in ["0", "-0", "", "   ", "+", "abc"] {
            assert_eq!(my_atoi(s), 0, "input {s:?}");
        }
    }

    #[test]
    fn the_core_keeps_them_apart() {
        assert_eq!(parse_int32_prefix("0"), Ok(0));
        assert_eq!(parse_int32_prefix("-0"), Ok(0));
        assert_eq!(parse_int32_prefix("words and 987"), Err(NoDigits));
        assert_eq!(parse_int32_prefix("abc"), Err(NoDigits));
        assert_eq!(parse_int32_prefix(""), Err(NoDigits));
        assert_eq!(parse_int32_prefix("   "), Err(NoDigits));
        assert_eq!(parse_int32_prefix("+"), Err(NoDigits));
        assert_eq!(parse_int32_prefix("-"), Err(NoDigits));
        assert_eq!(parse_int32_prefix("+-12"), Err(NoDigits));
    }

    #[test]
    fn clamp_is_also_two_answers_from_atoi() {
        assert_eq!(my_atoi("2147483647"), i32::MAX);
        assert_eq!(my_atoi("99999999999"), i32::MAX);
        assert_eq!(parse_int32_prefix("2147483647"), Ok(i32::MAX));
        assert_eq!(
            parse_int32_prefix("99999999999"),
            Err(OutOfRange { negative: false })
        );
    }

    #[test]
    fn clamping_both_ends() {
        assert_eq!(my_atoi("2147483648"), i32::MAX);
        assert_eq!(my_atoi("-2147483649"), i32::MIN);
        assert_eq!(my_atoi("-2147483648"), i32::MIN);
        // Reached by accumulating downward; no i64 and no abs() anywhere.
        assert_eq!(parse_int32_prefix("-2147483648"), Ok(i32::MIN));
        assert_eq!(
            parse_int32_prefix("2147483648"),
            Err(OutOfRange { negative: false })
        );
        assert_eq!(
            parse_int32_prefix("-2147483649"),
            Err(OutOfRange { negative: true })
        );
    }

    #[test]
    fn the_prefix_rule() {
        assert_eq!(my_atoi(" +0 123"), 0);
        assert_eq!(parse_int32_prefix(" +0 123"), Ok(0));
        assert_eq!(my_atoi("30s"), 30); // the unit disappears without a word
        assert_eq!(my_atoi("1-2"), 1);
    }

    #[test]
    fn leading_zeros_do_not_count_toward_the_range() {
        assert_eq!(my_atoi("0000000000012345678"), 12345678);
        assert_eq!(my_atoi("-000000000002147483648"), i32::MIN);
    }

    #[test]
    fn only_the_space_character_is_skipped() {
        assert_eq!(parse_int32_prefix("\t42"), Err(NoDigits));
        assert_eq!(parse_int32_prefix(&format!("{NBSP}42")), Err(NoDigits));
        assert_eq!(my_atoi("\n42"), 0);
        // trim_start() would have skipped them: it removes Unicode whitespace.
        assert_eq!(format!("{NBSP}\t42").trim_start(), "42");
    }

    #[test]
    fn only_ascii_digits_count() {
        assert_eq!(parse_int32_prefix(ARABIC_42), Err(NoDigits));
        assert_eq!(my_atoi(ARABIC_42), 0);
        assert_eq!(parse_int32_prefix("4\u{b2}"), Ok(4)); // "4²": the superscript ends the prefix
        // char::is_numeric is Unicode-wide; to_digit(10) is ASCII-only.
        assert!('\u{664}'.is_numeric());
        assert_eq!('\u{664}'.to_digit(10), None);
    }

    #[test]
    fn std_parse_says_no_and_says_why() {
        let kind = |s: &str| *s.parse::<i32>().unwrap_err().kind();
        // The whole string or nothing: no prefix rule, no space skipping.
        assert_eq!(kind("4193 with words"), IntErrorKind::InvalidDigit);
        assert_eq!(kind(" 42"), IntErrorKind::InvalidDigit);
        assert_eq!(kind("+"), IntErrorKind::InvalidDigit);
        assert_eq!(kind(""), IntErrorKind::Empty);
        assert_eq!(kind(ARABIC_42), IntErrorKind::InvalidDigit);
        // Too big and too small are values you can match on, not message text.
        assert_eq!(kind("99999999999"), IntErrorKind::PosOverflow);
        assert_eq!(kind("-2147483649"), IntErrorKind::NegOverflow);
        assert_eq!("-2147483648".parse::<i32>(), Ok(i32::MIN));
    }

    #[test]
    fn the_error_travels_up_with_question_mark() {
        fn port(s: &str) -> Result<u16, Box<dyn std::error::Error>> {
            Ok(u16::try_from(parse_int32_prefix(s)?)?)
        }
        assert_eq!(port("8080").unwrap(), 8080);
        assert_eq!(port("abc").unwrap_err().to_string(), "no digits");
    }

    #[test]
    fn agrees_with_an_i64_reference_on_ascii_inputs() {
        // The reference widens to i64 and clamps, which the solution never does.
        fn reference(s: &str) -> i32 {
            let t = s.trim_start_matches(' ');
            let (neg, rest) = match t.as_bytes().first() {
                Some(b'-') => (true, &t[1..]),
                Some(b'+') => (false, &t[1..]),
                _ => (false, t),
            };
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            let mut v: i64 = 0;
            for c in digits.chars() {
                v = (v * 10 + i64::from(c.to_digit(10).unwrap())).min(1 << 32);
            }
            let v = if neg { -v } else { v };
            v.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
        }

        let alphabet = b" +-0123456789a";
        let mut state: u64 = 20261116;
        let mut next = move |m: u64| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((state >> 33) % m) as usize
        };
        for _ in 0..100_000 {
            let len = next(15);
            let s: String = (0..len)
                .map(|_| alphabet[next(alphabet.len() as u64)] as char)
                .collect();
            assert_eq!(my_atoi(&s), reference(&s), "input {s:?}");
        }
    }
}
