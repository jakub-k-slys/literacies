//! Reverse Integer with the 32-bit boundary checked before it is crossed.
//!
//! Coding Literacy - overflow is a language decision.
//!
//! The loop is the same three steps as the Python and Scala siblings: pop the
//! last digit, drop it, push it onto the result. What happens on the push that
//! crosses 2^31 is not in the algorithm. In Rust, `rev * 10` on an `i32` is
//! governed by the `overflow-checks` setting: on in the dev profile, off in
//! release. So the same line panics with "attempt to multiply with overflow"
//! under `cargo test` and wraps silently under `cargo test --release`. Both are
//! defined behaviour, and they are different.
//!
//! `checked_mul` and `checked_add` take the choice out of the build profile and
//! put it in the type: they return `Option<i32>`, `None` means the result did not
//! fit, and the number cannot be used until `None` has been given a meaning,
//! which is the problem's "return 0" written down. No wider type appears in the
//! code.
//!
//! The sign stays in the digits. Rust's `%` truncates toward zero, so a negative
//! `x` yields negative digits and `rev` grows toward `i32::MIN` on its own. The
//! Python sibling's `abs(x)` is the trap here: `i32::MIN.abs()` panics in debug
//! builds and returns `i32::MIN` in release builds.

pub fn reverse(mut x: i32) -> i32 {
    let mut rev: i32 = 0;
    while x != 0 {
        let d = x % 10; // truncating: d carries x's sign, so no abs() and no i32::MIN trap
        x /= 10;
        // None is the problem's "return 0": the push would leave i32
        let Some(next) = rev.checked_mul(10).and_then(|r| r.checked_add(d)) else {
            return 0;
        };
        rev = next;
    }
    rev
}

/// The same loop with the guard missing, kept to show what the build profile
/// decides: a panic under the dev profile, a wrapped value under release.
pub fn reverse_unguarded(mut x: i32) -> i32 {
    let mut rev: i32 = 0;
    while x != 0 {
        let d = x % 10;
        x /= 10;
        rev = rev * 10 + d; // the line whose meaning depends on overflow-checks
    }
    rev
}

fn main() {
    // A tiny demo so `cargo run` does something visible.
    for sample in [123, -123, 120, 1534236469, i32::MIN] {
        println!("reverse({sample}) = {}", reverse(sample));
    }
}

#[cfg(test)]
mod tests {
    use super::{reverse, reverse_unguarded};
    use std::hint::black_box;

    #[test]
    fn statement_examples() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21); // 021 and 21 are the same integer
    }

    #[test]
    fn zero() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn the_reversal_that_needs_more_than_32_bits() {
        // 9646324351 does not fit; checked_mul refuses 964632435 * 10.
        assert_eq!(reverse(1534236469), 0);
    }

    #[test]
    fn minimum_value() {
        // The input abs() cannot handle in a fixed width.
        assert_eq!(reverse(i32::MIN), 0); // -2147483648
    }

    #[test]
    fn maximum_value() {
        assert_eq!(reverse(i32::MAX), 0); // 2147483647 -> 7463847412
    }

    #[test]
    fn the_only_inputs_that_reach_the_bound() {
        assert_eq!(reverse(1463847412), 2147483641);
        assert_eq!(reverse(-1463847412), -2147483641);
    }

    #[test]
    fn one_past_the_bound_is_refused() {
        assert_eq!(reverse(1563847412), 0); // 2147483651
        assert_eq!(reverse(-1563847412), 0);
        assert_eq!(reverse(1000000003), 0); // 3000000001
    }

    #[test]
    fn trailing_zeros_and_single_digits() {
        assert_eq!(reverse(-10), -1);
        assert_eq!(reverse(1000000000), 1);
        assert_eq!(reverse(7), 7);
        assert_eq!(reverse(-7), -7);
    }

    #[test]
    fn truncating_division_keeps_the_sign_in_the_digits() {
        // Python floors (-123 // 10 == -13, -123 % 10 == 7); Rust truncates.
        assert_eq!((-123 / 10, -123 % 10), (-12, -3));
        assert_eq!((-123i32).div_euclid(10), -13); // Python's answer, by name
        assert_eq!((-123i32).rem_euclid(10), 7);
        // The bound constant: truncating, so no off-by-one.
        assert_eq!(i32::MIN / 10, -214748364);
    }

    #[test]
    fn abs_of_the_minimum_does_not_fit() {
        // Why this version does not take abs(x) first, as the Python one does.
        assert_eq!(i32::MIN.checked_abs(), None);
        assert_eq!(i32::MIN.wrapping_abs(), i32::MIN);
    }

    // `cargo test` runs the dev profile, where overflow-checks is on, so the
    // unguarded loop panics. `cargo test --release` turns it off, and the same
    // loop wraps: 964632435 * 10 becomes 1056389758, then + 1. The default
    // profiles tie overflow-checks to debug-assertions, which is what the
    // attribute keys on.
    #[test]
    #[cfg_attr(debug_assertions, should_panic(expected = "attempt to multiply with overflow"))]
    fn unguarded_loop_panics_in_debug_and_wraps_in_release() {
        let got = reverse_unguarded(black_box(1534236469));
        assert_eq!(got, 1056389759); // positive, ten digits, and wrong
    }

    #[test]
    fn agrees_with_a_64_bit_oracle() {
        // The oracle widens to i64, which the solution is not allowed to do.
        fn oracle(x: i32) -> i32 {
            let s: String = x.unsigned_abs().to_string().chars().rev().collect();
            let r: i64 = s.parse::<i64>().unwrap() * i64::from(x.signum());
            i32::try_from(r).unwrap_or(0)
        }

        // A tiny deterministic LCG keeps this dependency-free.
        let mut state: u64 = 20261109;
        let mut next = move || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (state >> 32) as u32 as i32
        };
        for _ in 0..200_000 {
            let x = next();
            assert_eq!(reverse(x), oracle(x), "input {x}");
        }
        for x in [i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX] {
            assert_eq!(reverse(x), oracle(x), "input {x}");
        }
    }
}
