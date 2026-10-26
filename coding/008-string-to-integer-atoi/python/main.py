"""String to Integer (atoi) as a parser that can say no, O(n) time, O(1) space.

Coding Literacy - if zero is a valid answer, it cannot also be the error.

The judge's atoi returns 0 for "0" and for "words and 987", and 2147483647 for
"2147483647" and for "99999999999". One int is asked to carry a number and a
verdict on whether there was a number, and it has room for one of them.

So the code has two layers. parse_int32_prefix() is the core: it returns one
of three small frozen dataclasses, Parsed(value), NoDigits() or
OutOfRange(negative), so the no travels on a channel the value cannot occupy.
my_atoi() is the adapter the judge calls: a match over that result, and the
only place in the program that decides 0 means "nothing here" and that too big
means clamp.

Two rules are fixed on purpose. Whitespace is the space character ' ' and
nothing else (no lstrip(), which removes every Unicode space), and a digit is
'0' <= c <= '9' (no isdigit(), which is True for '²' and the Arabic-Indic '٤').

The overflow guard is Reverse Integer's: compare rev against limit // 10 and
the digit against limit % 10 before every push, where limit is 2147483647 for
a positive number and 2147483648 for a negative one. For "-2147483648" the
magnitude briefly reaches 2147483648, which leans on Python's unbounded int;
the Rust and Scala siblings keep the sign in the digits instead.

my_atoi_naive() is the loop most people write first. It passes the judge,
returns 0 for both "0" and "abc", raises ValueError on "4²" and turns
"\u00a042" (a non-breaking space, then 42) into 42.
"""

from dataclasses import dataclass


def my_atoi_naive(s: str) -> int:
    s = s.strip()  # strips all Unicode whitespace, not only ' '
    if not s:
        return 0
    sign, i = 1, 0
    if s[0] in "+-":
        sign = -1 if s[0] == "-" else 1
        i = 1
    num = 0
    while i < len(s) and s[i].isdigit():  # True for '²' and '٤' too
        num = num * 10 + int(s[i])
        i += 1
    return max(-(2**31), min(2**31 - 1, sign * num))


INT_MAX = 2**31 - 1  # 2147483647


@dataclass(frozen=True)
class Parsed:
    value: int


@dataclass(frozen=True)
class NoDigits:
    pass


@dataclass(frozen=True)
class OutOfRange:
    negative: bool


def parse_int32_prefix(s: str) -> Parsed | NoDigits | OutOfRange:
    i, n = 0, len(s)
    while i < n and s[i] == " ":  # the space character only, not lstrip()
        i += 1
    negative = False
    if i < n and s[i] in "+-":
        negative = s[i] == "-"
        i += 1
    limit = INT_MAX + 1 if negative else INT_MAX
    bound, last = divmod(limit, 10)  # 214748364, and 7 or 8
    start, rev = i, 0
    while i < n and "0" <= s[i] <= "9":  # ASCII digits only, not isdigit()
        d = ord(s[i]) - ord("0")
        if rev > bound or (rev == bound and d > last):  # refuse before the push
            return OutOfRange(negative)
        rev = rev * 10 + d
        i += 1
    if i == start:  # the index never moved past the sign
        return NoDigits()
    return Parsed(-rev if negative else rev)


def my_atoi(s: str) -> int:
    match parse_int32_prefix(s):
        case Parsed(value):
            return value
        case NoDigits():
            return 0  # the one place that decides 0 means "nothing here"
        case OutOfRange(negative):
            return -(INT_MAX + 1) if negative else INT_MAX


if __name__ == "__main__":
    # A tiny demo so `uv run main.py` does something visible.
    for sample in ("42", "   -42", "4193 with words", "words and 987", "0", "99999999999"):
        print(f"{sample!r:>18}  parse_int32_prefix = {parse_int32_prefix(sample)!s:<26} my_atoi = {my_atoi(sample)}")
