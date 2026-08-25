"""Longest palindromic substring by expansion around centers, O(n^2).

Coding Literacy - the idea is free, the notation is not.

A string of length n has 2n-1 centers: n single characters for the odd-length
palindromes, n-1 gaps between adjacent characters for the even-length ones. Each
center has exactly one maximal answer, reached by growing outward while
s[l] == s[r] holds.

The language-visible decision is that s[i:j] is not notation, it is an
allocation and a copy. So the expansion carries (best_start, best_len) as plain
integers and slices exactly once, on return.
"""


def expand(s: str, l: int, r: int) -> tuple[int, int]:
    while l >= 0 and r < len(s) and s[l] == s[r]:
        l -= 1
        r += 1
    return l, r  # bounds, not a substring: both ends overshot by one


def longest_palindrome(s: str) -> str:
    best_start, best_len = 0, 0
    for i in range(len(s)):
        for l, r in (expand(s, i, i), expand(s, i, i + 1)):  # odd center, then gap center
            if r - l - 1 > best_len:
                best_start, best_len = l + 1, r - l - 1
    return s[best_start : best_start + best_len]  # the whole run's only string allocation


if __name__ == "__main__":
    # A tiny demo so `uv run main.py` does something visible.
    for sample in ("babad", "cbbd", "forgeeksskeegfor"):
        print(f"longest_palindrome({sample!r}) = {longest_palindrome(sample)!r}")
