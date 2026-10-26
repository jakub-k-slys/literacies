"""Tests for parse_int32_prefix, my_atoi and my_atoi_naive.

Run: uv run pytest
"""
import pytest

from main import (
    INT_MAX,
    NoDigits,
    OutOfRange,
    Parsed,
    my_atoi,
    my_atoi_naive,
    parse_int32_prefix,
)

INT_MIN = -INT_MAX - 1

NBSP = "\u00a0"  # a non-breaking space, kept as an escape on purpose


def test_statement_examples():
    for f in (my_atoi, my_atoi_naive):
        assert f("42") == 42
        assert f(" -042") == -42
        assert f("1337c0d3") == 1337
        assert f("0-1") == 0
        assert f("words and 987") == 0
        assert f("-91283472332") == INT_MIN  # earlier statement example: below INT_MIN, clamps


def test_earlier_statement_examples():
    for f in (my_atoi, my_atoi_naive):
        assert f("   -42") == -42
        assert f("4193 with words") == 4193
        assert f("0") == 0


def test_zero_and_no_digits_are_one_answer_from_atoi():
    # Six inputs, one int: only "0" and "-0" contained a number.
    for s in ("0", "-0", "", "   ", "+", "abc"):
        assert my_atoi(s) == 0
        assert my_atoi_naive(s) == 0


def test_the_core_keeps_them_apart():
    assert parse_int32_prefix("0") == Parsed(0)
    assert parse_int32_prefix("-0") == Parsed(0)
    assert parse_int32_prefix("words and 987") == NoDigits()
    assert parse_int32_prefix("abc") == NoDigits()
    assert parse_int32_prefix("") == NoDigits()
    assert parse_int32_prefix("   ") == NoDigits()
    assert parse_int32_prefix("+") == NoDigits()
    assert parse_int32_prefix("-") == NoDigits()
    assert parse_int32_prefix("+-12") == NoDigits()  # one sign, then a non-digit


def test_clamp_is_also_two_answers_from_atoi():
    assert my_atoi("2147483647") == INT_MAX
    assert my_atoi("99999999999") == INT_MAX
    assert parse_int32_prefix("2147483647") == Parsed(INT_MAX)
    assert parse_int32_prefix("99999999999") == OutOfRange(negative=False)


def test_clamping_both_ends():
    assert my_atoi("2147483648") == INT_MAX
    assert my_atoi("-2147483649") == INT_MIN
    assert my_atoi("-2147483648") == INT_MIN
    assert parse_int32_prefix("-2147483648") == Parsed(INT_MIN)
    assert parse_int32_prefix("-91283472332") == OutOfRange(negative=True)


def test_the_tie_break_fires():
    # rev reaches 214748364 and the next digit decides: 7 fits on the positive
    # side, 8 only on the negative side.
    assert parse_int32_prefix("2147483648") == OutOfRange(negative=False)
    assert parse_int32_prefix("-2147483649") == OutOfRange(negative=True)
    assert parse_int32_prefix("2147483657") == OutOfRange(negative=False)  # rev 214748365 is past the bound
    assert parse_int32_prefix("21474836470") == OutOfRange(negative=False)


def test_the_prefix_rule():
    assert parse_int32_prefix("1337c0d3") == Parsed(1337)
    assert parse_int32_prefix("0-1") == Parsed(0)
    assert parse_int32_prefix(" -042") == Parsed(-42)
    assert parse_int32_prefix("30s") == Parsed(30)
    assert my_atoi(" +0 123") == 0
    assert parse_int32_prefix(" +0 123") == Parsed(0)
    assert my_atoi("30s") == 30  # the unit disappears without a word
    assert my_atoi("1-2") == 1


def test_leading_zeros_do_not_count_toward_the_range():
    assert my_atoi("0000000000012345678") == 12345678
    assert my_atoi("-000000000002147483648") == INT_MIN


def test_only_the_space_character_is_skipped():
    assert parse_int32_prefix("\t42") == NoDigits()
    assert parse_int32_prefix(f"{NBSP}42") == NoDigits()
    assert my_atoi(f"{NBSP}42") == 0
    assert my_atoi("\n42") == 0
    # The naive version's strip() removes all Unicode whitespace.
    assert my_atoi_naive(f"{NBSP}42") == 42
    assert my_atoi_naive("\t42") == 42


def test_only_ascii_digits_count():
    arabic_42 = "\u0664\u0662"  # "٤٢", Arabic-Indic four and two
    assert parse_int32_prefix(arabic_42) == NoDigits()
    assert my_atoi(arabic_42) == 0
    assert parse_int32_prefix("4\u00b2") == Parsed(4)  # "4²": the superscript ends the prefix
    # What an isdigit() test would have let in.
    assert "\u0664".isdigit() and "\u00b2".isdigit()
    assert ord("\u0664") - ord("0") == 1588


def test_the_naive_loop_breaks_its_own_never_fails_contract():
    with pytest.raises(ValueError):
        my_atoi_naive("4\u00b2")  # "²".isdigit() is True, int("²") raises


def test_python_int_is_strict_and_unicode_aware():
    # The standard parser reads the whole string or refuses, on a separate channel.
    with pytest.raises(ValueError):
        int("1337c0d3")
    with pytest.raises(ValueError):
        int("4193 with words")
    with pytest.raises(ValueError):
        int("+")
    # It accepts any Unicode decimal digit and strips Unicode whitespace.
    assert int("\u0664\u0662") == 42
    assert int(f"{NBSP}42") == 42


def test_adapter_agrees_with_a_reference_on_ascii_inputs():
    import random
    import re

    def reference(s: str) -> int:
        m = re.match(r" *([+-]?[0-9]+)", s)
        return max(INT_MIN, min(INT_MAX, int(m.group(1)))) if m else 0

    rng = random.Random(20261116)
    alphabet = " +-0123456789a"
    for _ in range(20000):
        s = "".join(rng.choice(alphabet) for _ in range(rng.randint(0, 14)))
        assert my_atoi(s) == reference(s), repr(s)
