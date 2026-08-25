# 005 · Longest Palindromic Substring — Rust

`O(n^2)` expansion around centers, returning a **borrowed** `&str`.

## Idea

A string of `n` characters has `2n-1` centers: `n` single characters for the
odd-length palindromes, `n-1` gaps for the even-length ones. Grow outward from
each while both ends match, keep the best bounds. The first mismatch is the last
word for that center, so the walk never backtracks.

The language-visible decision here is the **unit**. A `&str` is a borrowed view
into UTF-8 bytes, so `&s[i..j]` copies nothing, and Rust takes back the
guarantee that `i` counts characters. Walking bytes does not just run slower on
non-ASCII input, it returns the wrong answer: `"óaó"` is a palindrome by
characters but not by bytes (`C3 B3 61 C3 B3` reversed is `B3 C3 61 B3 C3`). And
on `"aóòa"` a byte-level walk picks bytes `1..4`, where slicing panics with
`byte index 4 is not a char boundary`. Wrong answer and a crash on the same
input.

Two fixes exist and they cost different things. Collecting a `Vec<char>` is
correct but gives the borrow up, because `Vec<char>` is an owned copy at four
bytes per character. Walking a `char_indices` table, which is what this project
does, **keeps the borrow** and pays for one index walk instead: the return type
is `&str`, pointing into the caller's own buffer, and no `String` is ever built.
`the_result_borrows_the_input_and_never_copies` asserts exactly that, by
comparing pointers.

## Run

```
cargo test        # 12 tests
cargo run         # tiny demo
```

## Files

- `src/main.rs` — `longest_palindrome(&str) -> &str`, an `expand` helper returning bounds, and `#[cfg(test)]`
