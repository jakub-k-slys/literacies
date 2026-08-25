"""Tests for longest_palindrome.

Run: uv run pytest
"""
from main import longest_palindrome as lps


def test_statement_examples():
    assert lps("babad") in ("bab", "aba")  # ties are allowed, the length is what is fixed
    assert lps("cbbd") == "bb"


def test_single_character():
    assert lps("a") == "a"


def test_empty():
    assert lps("") == ""


def test_no_palindrome_longer_than_one():
    assert len(lps("abcde")) == 1


def test_even_length_needs_the_gap_centers():
    # Drop the gap centers and this returns a single character instead.
    assert lps("abba") == "abba"
    assert lps("bb") == "bb"


def test_whole_string_is_the_answer():
    assert lps("racecar") == "racecar"
    assert lps("aaaa") == "aaaa"


def test_answer_is_interior():
    assert lps("forgeeksskeegfor") == "geeksskeeg"


def test_off_by_one_at_the_edges():
    # The winning window touching either end is where the s[l+1 : r] arithmetic
    # is easiest to get wrong by one character.
    assert lps("aab") == "aa"
    assert lps("baa") == "aa"


def test_python_counts_code_points_not_bytes():
    # The language-visible difference. Python indexes code points, so the naive
    # two-pointer walk is correct here with no extra work. The same walk over
    # UTF-8 bytes in Rust is wrong, and over UTF-16 code units on the JVM it is
    # wrong for non-BMP characters. See the sibling rust/ and scala/ projects.
    assert lps("óaó") == "óaó"
    assert lps("aóòa") == "a"  # no palindrome longer than one character here


def test_agrees_with_brute_force():
    import itertools
    import random

    def brute(s: str) -> int:
        best = 0
        for i, j in itertools.combinations(range(len(s) + 1), 2):
            w = s[i:j]
            if w == w[::-1]:
                best = max(best, len(w))
        return best

    rng = random.Random(20260825)
    for _ in range(500):
        s = "".join(rng.choice("abc") for _ in range(rng.randrange(0, 15)))
        got = lps(s)
        assert len(got) == brute(s)
        assert got == got[::-1]
        assert got in s
