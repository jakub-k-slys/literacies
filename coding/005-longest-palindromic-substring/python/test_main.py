"""Tests for longest_palindrome.

Run: uv run pytest
"""
from main import expand, longest_palindrome as lps


def test_expand_returns_bounds_not_a_substring():
    # The contract the whole design rests on: the helper hands back bounds, so
    # nothing is materialized per anchor. Both ends overshoot by one.
    assert expand("aba", 1, 1) == (-1, 3)  # odd center on 'b', grows to the whole string
    assert expand("ab", 0, 1) == (0, 1)  # gap center, immediate mismatch
    assert expand("aa", 0, 1) == (-1, 2)  # gap center, grows to the whole string


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
    # The language-visible difference. Python indexes code points, so the walk
    # needs no extra work at that level. The same walk over UTF-8 bytes in Rust
    # is wrong, and over UTF-16 code units on the JVM it is wrong for non-BMP
    # characters. See the sibling rust/ and scala/ projects.
    assert lps("óaó") == "óaó"
    assert lps("aóòa") == "a"  # no palindrome longer than one character here


def test_code_points_are_still_not_grapheme_clusters():
    # The limit all three siblings share, pinned rather than left to chance. An
    # accented e written decomposed is e + U+0301, so two of them are the four
    # code points e U+0301 e U+0301. By code points the longest palindrome is
    # three long and tears a combining mark off its base character. Grapheme
    # clusters are the next unit up, and none of the three siblings reaches for
    # them. Two windows tie at three, so pin the length, not which tie wins.
    decomposed = "e\u0301e\u0301"
    got = lps(decomposed)
    assert len(got) == 3
    assert got.startswith("\u0301") or not got.endswith("\u0301")


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

    # The alphabet mixes 1, 2 and 4-byte characters, matching the Rust sibling,
    # so the byte/character split is exercised even though Python hides it.
    alphabet = ["a", "b", "ó", "ò", "\U0001f600"]
    rng = random.Random(20260825)
    for _ in range(2000):
        s = "".join(rng.choice(alphabet) for _ in range(rng.randrange(0, 15)))
        got = lps(s)
        assert len(got) == brute(s)
        assert got == got[::-1]
        assert got in s
