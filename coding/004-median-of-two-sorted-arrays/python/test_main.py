"""Tests for find_median_sorted_arrays.

Run: uv run pytest
"""
from main import find_median_sorted_arrays as med


def test_odd_total():
    assert med([1, 3], [2]) == 2.0


def test_even_total():
    assert med([1, 2], [3, 4]) == 2.5


def test_one_array_empty():
    assert med([], [1]) == 1.0
    assert med([2, 4, 6], []) == 4.0


def test_very_unbalanced():
    assert med([1], [2, 3, 4, 5, 6]) == 3.5


def test_duplicates_and_equal_boundaries():
    assert med([1, 1], [1, 1]) == 1.0
    assert med([1, 2, 2, 3], [2, 2]) == 2.0


def test_negative_and_mixed():
    assert med([-5, -3, -1], [-2, 0, 2]) == -1.5
