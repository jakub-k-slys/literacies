# 008 · String to Integer (atoi) — Python

One pass over the string, `O(n)` time, `O(1)` extra space, and a result that can
say **no** on a channel the value cannot occupy.

## Idea

The judge's atoi returns `0` for `"0"` and for `"words and 987"`, and
`2147483647` for `"2147483647"` and for `"99999999999"`. Every one of the 2^32
values of a 32-bit int is somebody's correct answer, so no sentinel is left
over: the *no* has to travel outside the value. If zero means something, it
cannot also mean error.

So there are two layers:

- `parse_int32_prefix(s)` is the core. It returns one of three frozen
  dataclasses: `Parsed(value)`, `NoDigits()` or `OutOfRange(negative)`.
  `"0"` is `Parsed(0)`, `"abc"` is `NoDigits()`, `"99999999999"` is
  `OutOfRange(False)`.
- `my_atoi(s)` is the adapter the judge calls: a `match` over that result.
  `NoDigits()` maps to 0 and `OutOfRange` to the clamp, in one visible place.

Two rules are fixed on purpose:

- **Whitespace is the space character `' '` only.** An index loop, not
  `lstrip()`, which removes every kind of Unicode whitespace. A tab or a
  non-breaking space (`"\u00a042"`) is `NoDigits()`.
- **A digit is `'0' <= c <= '9'`**, and its value is `ord(c) - ord('0')`.
  `str.isdigit()` is True for `'²'` and for the Arabic-Indic `'٤'`, and
  `ord('٤') - ord('0')` is 1588. With the ASCII test `"٤٢"` is `NoDigits()`;
  Python's own `int("٤٢")` is 42, which a test pins as the contrast.

The overflow guard is Reverse Integer's (007), and here its tie-break fires:
`rev` is compared against `limit // 10` and the digit against `limit % 10`
before every push, with `limit` 2147483647 for a positive number and 2147483648
for a negative one. `"2147483648"` reaches `rev == 214748364` with a final 8, one
more than 7 allows. For `"-2147483648"` the magnitude briefly reaches
2147483648, which leans on Python's unbounded `int`; the Rust and Scala siblings
keep the sign in the digits instead.

`my_atoi_naive()` is the loop most people write first: `strip()`, a sign,
`isdigit()`, `int(c)`, then `max`/`min`. It passes the judge, returns 0 for both
`"0"` and `"abc"`, raises `ValueError` on `"4²"` (`"²".isdigit()` is True,
`int("²")` raises), and turns `"\u00a042"` into 42.

## Run

```
uv sync                # install deps into .venv
uv run pytest          # 13 tests
uv run main.py         # tiny demo
```

## Files

- `main.py` — `parse_int32_prefix(s) -> Parsed | NoDigits | OutOfRange` (the core), `my_atoi(s) -> int` (the judge's adapter), `my_atoi_naive(s) -> int` (the first-try loop)
- `test_main.py` — statement examples, the six inputs that all give 0, the core keeping them apart, clamping at both ends, the tie-break, the prefix rule (`" +0 123"`, `"30s"`), leading zeros, tab and NBSP not skipped, `"٤٢"` rejected, the naive loop's `ValueError`, `int()` as the strict Unicode-aware contrast, randomized cross-check against a regex reference
