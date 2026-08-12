# 004 · Median of Two Sorted Arrays — Rust

`O(log min(m, n))` partition / binary-search solution.

## Idea

You do not *reach* the median by walking the merged data, you *place* a boundary.
Choose how many elements come from the smaller slice's left side; that single
choice forces the cut in the other slice. Binary-search the choice until the left
half is entirely `<=` the right half.

The language-visible decision here is **`i64` widening**: inputs are `i32`, but a
debug build panics on `i32` overflow when you sum two boundary values to average
them, so the boundaries are lifted to `i64` first. The `+/- inf` sentinels become
`i64::MIN` / `i64::MAX`.

## Run

```
cargo test        # 8 tests
cargo run         # tiny demo
```

## Files

- `src/main.rs` — `find_median_sorted_arrays(&[i32], &[i32]) -> f64` + `#[cfg(test)]`
