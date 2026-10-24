# 007 · Reverse Integer — Rust

One pass over the digits (at most 10 for an `i32`), `O(1)` extra state, every
push made through `checked_mul` / `checked_add`.

## Idea

Pop the last digit, drop it, push it onto the result. The push is the only step
that can leave the 32-bit range, and the problem's answer for that case is 0.

The language-visible decision here is the **build profile**. `rev * 10` on an
`i32` is governed by `overflow-checks`: on in the dev profile, off in release.
So the plain line panics with `attempt to multiply with overflow` under
`cargo test` and wraps silently under `cargo test --release`: `964632435 * 10`
becomes `1056389758`, positive, ten digits, and wrong. Neither is undefined
behaviour, the way signed overflow is in C. Both are defined, and they differ.
`reverse_unguarded` keeps that version around, and
`unguarded_loop_panics_in_debug_and_wraps_in_release` pins both outcomes, one
per profile, so **run the tests under both**.

`checked_mul` and `checked_add` take the choice out of the build profile and put
it in the type. They return `Option<i32>`; `None` means the result did not fit,
and the `let ... else` cannot go on until `None` has a meaning, which is the
problem's "return 0" written down. No wider type appears in the code.
(`wrapping_*`, `saturating_*` and `overflowing_*` are the other named choices.)

The sign stays in the digits. Rust's `/` and `%` truncate toward zero, so a
negative `x` yields negative digits and `rev` grows toward `i32::MIN` by itself.
The Python sibling's `abs(x)`-first move is an overflow here: `i32::MIN.abs()`
panics in debug builds and returns `i32::MIN` in release builds.

## Run

```
cargo test             # 12 tests, dev profile (overflow-checks on)
cargo test --release   # the same 12, release profile (overflow-checks off)
cargo run              # tiny demo
```

## Files

- `src/main.rs` — `reverse(i32) -> i32` with `checked_mul` / `checked_add`, `reverse_unguarded` (the profile-dependent version), and `#[cfg(test)]`, including a randomized cross-check against an `i64` oracle
