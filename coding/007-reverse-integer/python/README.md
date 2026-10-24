# 007 · Reverse Integer — Python

One pass over the digits (at most 10 for a 32-bit value), `O(1)` extra state,
the 32-bit boundary checked **before** every push.

## Idea

The loop is three steps: pop the last digit (`x % 10`), drop it (`x // 10`),
push it onto the result (`rev * 10 + d`). The push is the only step that can
leave the 32-bit range, so `reverse()` refuses it before making it: `rev` is
compared against `limit // 10` and the digit against `limit % 10`, where `limit`
is `2147483647` for a positive `x` and `2147483648` for a negative one. The
comparison runs on values that already fit, so `rev` never leaves the range.

The language-visible decision here is that **Python's `int` has no width**. The
push that crosses 2^31 simply succeeds, and the range exists only because you
wrote the check. `reverse_via_string()` is the short version most people write
first: correct, accepted by the judge, and for `1534236469` it builds
`9646324351`, the value the problem says you cannot store, and range-checks it
after the fact. Both versions are here; the tests cross-check them.

Python's `//` and `%` **floor** toward negative infinity, where Rust and the JVM
truncate toward zero: `-123 // 10 == -13`, `-123 % 10 == 7`, and `-1 // 10 == -1`,
so a line-by-line port of a truncating `while x != 0` loop never terminates on a
negative input. The bound constant is a division too: `-2**31 // 10` is
`-214748365`, one past the truncating languages' `-214748364`. So the loop works
on `abs(x)` and reapplies the sign. That is correct in Python and an overflow in
the fixed-width siblings (`abs` of the minimum value), which is why `../rust` and
`../scala` keep the sign in the digits instead.

The tie-break (`rev == bound and d > last`) never fires for an input in the
32-bit range: the final digit pushed is the input's leading digit, at most 2. It
stays because it is correct and free, and String to Integer (atoi) needs it;
`test_tie_break_outside_the_problem_range` exercises it on purpose.

## Run

```
uv sync                # install deps into .venv
uv run pytest          # 11 tests
uv run main.py         # tiny demo
```

## Files

- `main.py` — `reverse(x) -> int` (guarded digit-pop) and `reverse_via_string(x) -> int` (the string version)
- `test_main.py` — statement examples, zero, `-2147483648`, `2147483647`, `1534236469`, the bound inputs `±1463847412`, one past the bound, the tie-break outside the range, floor-division facts, randomized cross-check of the two versions
