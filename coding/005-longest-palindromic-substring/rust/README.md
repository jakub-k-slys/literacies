# 005 · Longest Palindromic Substring — Rust

`O(n^2)` expansion around centers, allocating nothing and returning a
**borrowed** `&str`.

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

The usual fix, collecting a `Vec<char>`, is correct and gives the borrow up. Not
because collecting is inherently expensive, but because a `Vec<char>` has
**not recorded the byte offsets**: it can tell you the answer runs from character
3 to character 9, and recovering where that is in the original buffer costs
another pass summing `len_utf8()`, so in practice the answer gets rebuilt as an
owned `String`.

This project keeps the offsets instead. `char_indices` yields byte offsets that
are already character boundaries, and `chars().next_back()` / `chars().next()`
step one character out from a boundary in constant time, because UTF-8 is
self-synchronizing. So the walk carries byte offsets throughout, **allocates
nothing at all**, and ends with one `&s[lo..hi]` pointing into the caller's
string. `the_result_borrows_the_input_and_never_copies` asserts that by comparing
pointers, including for the empty answer.

One consequence worth stating, since the whole project is about picking a unit
on purpose: the expansion tracks its length **in characters**, not in bytes.
Ranking candidates by byte length would put a single four-byte emoji above a
genuine two-character palindrome. The randomized cross-check caught exactly that.

Characters are still not the last word. Code points are not grapheme clusters, so
a combining mark can be torn off its base character. `code_points_are_still_not_grapheme_clusters`
pins that limit rather than leaving it to chance; all three siblings share it.

## Run

```
cargo test        # 14 tests
cargo run         # tiny demo
```

## Files

- `src/main.rs` — `longest_palindrome(&str) -> &str`, an `expand` helper returning byte bounds plus a character count, and `#[cfg(test)]`
