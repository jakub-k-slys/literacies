# 008 · String to Integer (atoi) — Rust

One pass over the bytes, `O(n)` time, `O(1)` extra space, and a result that can
say **no** on a channel the value cannot occupy.

## Idea

The judge's atoi returns `0` for `"0"` and for `"words and 987"`, and
`i32::MAX` for `"2147483647"` and for `"99999999999"`. Every `i32` is somebody's
correct answer, so the *no* has to travel outside the value.

`parse_int32_prefix(&str) -> Result<i32, PrefixError>` is the core, with

```rust
pub enum PrefixError {
    NoDigits,
    OutOfRange { negative: bool },
}
```

`Ok(value)` is the Python sibling's `Parsed(value)`; the two errors are its
`NoDigits()` and `OutOfRange(negative)`. Rust spells a three-way parse as
`Result` plus an error enum, so the outcome works with `match` and with `?`
(`PrefixError` implements `std::error::Error`). `my_atoi(&str) -> i32` is the
adapter the judge calls, and its `NoDigits => 0` arm is the only place that
decides 0 means "nothing here".

The grammar is ASCII and runs over `s.as_bytes()`: the space byte `b' '` only
(not `trim_start()`, which removes Unicode whitespace), one optional sign,
digits `b'0'..=b'9'`. That is safe on any `&str`, because every byte of a
multi-byte UTF-8 character is `0x80` or above, so a non-breaking space or the
Arabic-Indic `"٤٢"` can never be read as a space or a digit: both are
`NoDigits`.

The sign stays in the digits. A negative number accumulates downward, so
`"-2147483648"` reaches `i32::MIN` without an `i64` and without `abs()`, the
end the Python sibling handles by briefly holding 2147483648. Every push goes
through `checked_mul` / `checked_add`, and `None` becomes
`OutOfRange { negative }`: Reverse Integer's guard, with the tie-break done by
the type. Overflow is checked by the code, so the result is the same under
`cargo test` and `cargo test --release`.

The standard library makes the same move one level up. `str::parse::<i32>()`
returns a `Result` whose `ParseIntError::kind()` separates `Empty`,
`InvalidDigit`, `PosOverflow` and `NegOverflow`, values you can match on. It
reads the whole string or refuses, so `"4193 with words"` is `InvalidDigit`, and
unlike Python's `int()` or the JVM's parser it rejects `"٤٢"`.

## Run

```
cargo test             # 13 tests, dev profile
cargo test --release   # the same 13, release profile
cargo run              # tiny demo
```

## Files

- `src/lib.rs` — `PrefixError`, `parse_int32_prefix`, `my_atoi`, and `#[cfg(test)]`: statement examples, the six inputs that all give 0, the core keeping them apart, clamping at both ends, the prefix rule, leading zeros, tab and NBSP not skipped, `"٤٢"` rejected, `ParseIntError::kind()` facts, `?` carrying the error up, randomized cross-check against an `i64` reference
- `src/main.rs` — tiny demo
