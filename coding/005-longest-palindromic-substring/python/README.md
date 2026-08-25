# 005 · Longest Palindromic Substring — Python

`O(n^2)` expansion around centers, `O(1)` extra state.

## Idea

A string of length `n` has `2n-1` centers: `n` single characters for the
odd-length palindromes, `n-1` gaps between adjacent characters for the
even-length ones. Each center has exactly one maximal answer, reached by growing
outward while `s[l] == s[r]` holds. The first mismatch is the last word for that
center, so the walk never backtracks.

The language-visible decision here is that **`s[i:j]` is an allocation and a
copy**, not notation. CPython shares a slice in only three narrow cases: a slice
covering the whole string, an empty slice, and a length-1 slice of a Latin-1
character. So the expansion carries `(best_start, best_len)` as plain integers
and slices exactly once, on return; the helper returns bounds rather than a
substring for the same reason.

Python indexes **code points**, so the two-pointer walk is correct on non-ASCII
input with no extra work. That is not free elsewhere: see `../rust` (byte
offsets) and `../scala` (UTF-16 code units).

## Run

```
uv sync                # install deps into .venv
uv run pytest          # 10 tests
uv run main.py         # tiny demo
```

## Files

- `main.py` — `expand(s, l, r) -> (int, int)` and `longest_palindrome(s) -> str`
- `test_main.py` — statement examples, empty, gap centers, edge windows, non-ASCII, randomized cross-check against brute force
