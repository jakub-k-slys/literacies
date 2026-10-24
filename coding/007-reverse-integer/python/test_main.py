"""Tests for reverse and reverse_via_string.

Run: uv run pytest
"""
import random

from main import INT_MAX, reverse, reverse_via_string

INT_MIN = -INT_MAX - 1

BOTH = (reverse, reverse_via_string)


def test_statement_examples():
    for f in BOTH:
        assert f(123) == 321
        assert f(-123) == -321
        assert f(120) == 21  # 021 and 21 are the same integer


def test_zero():
    for f in BOTH:
        assert f(0) == 0


def test_the_reversal_that_needs_more_than_32_bits():
    # 9646324351 does not fit. reverse() stops after nine pushes, at rev ==
    # 964632435, which is already past 214748364; the string version builds the
    # ten-digit value first and range-checks it afterwards.
    for f in BOTH:
        assert f(1534236469) == 0


def test_minimum_value():
    # -2147483648 reverses to -8463847412, far outside the range.
    for f in BOTH:
        assert f(INT_MIN) == 0


def test_maximum_value():
    for f in BOTH:
        assert f(INT_MAX) == 0  # 7463847412


def test_the_only_inputs_that_reach_the_bound():
    # rev == 214748364 before the final push, final digit 1: fits on both sides.
    for f in BOTH:
        assert f(1463847412) == 2147483641
        assert f(-1463847412) == -2147483641


def test_one_past_the_bound_is_refused():
    for f in BOTH:
        assert f(1563847412) == 0  # 2147483651
        assert f(-1563847412) == 0
        assert f(1000000003) == 0  # 3000000001


def test_trailing_zeros_and_single_digits():
    for f in BOTH:
        assert f(-10) == -1
        assert f(1000000000) == 1
        assert f(7) == 7
        assert f(-7) == -7


def test_tie_break_outside_the_problem_range():
    # On 007 the tie-break never fires for a valid input. It is kept because the
    # same guard is reused where the input is arbitrary (008, atoi); these inputs
    # are outside the 32-bit range on purpose, to show it is correct.
    assert reverse(7463847412) == INT_MAX  # rev == bound, d == 7: allowed
    assert reverse(8463847412) == 0  # rev == bound, d == 8 > 7: refused
    assert reverse(-8463847412) == INT_MIN  # the negative side allows the 8
    assert reverse(-9463847412) == 0


def test_floor_division_is_why_the_loop_runs_on_abs():
    # A line-by-line port of a truncating loop would pop the wrong digits and,
    # since -1 // 10 == -1, never reach zero.
    assert divmod(-123, 10) == (-13, 7)
    assert -1 // 10 == -1
    # The bound constant is a division too: floor gives one past the truncating
    # languages' INT_MIN / 10 == -214748364.
    assert INT_MIN // 10 == -214748365


def test_agrees_with_the_string_version():
    rng = random.Random(20261109)
    samples = [rng.randint(INT_MIN, INT_MAX) for _ in range(20000)]
    samples += [rng.randint(-(10**k), 10**k) for k in range(1, 10) for _ in range(500)]
    for x in samples:
        assert reverse(x) == reverse_via_string(x), x

