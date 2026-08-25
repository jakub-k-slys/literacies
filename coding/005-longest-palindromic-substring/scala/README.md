# 005 · Longest Palindromic Substring — Scala

`O(n^2)` expansion around centers. An sbt project.

## Idea

A string of `n` characters has `2n-1` centers: `n` single characters for the
odd-length palindromes, `n-1` gaps for the even-length ones. Grow outward from
each while both ends match, keep the best bounds. The first mismatch is the last
word for that center, so the walk never backtracks.

The language-visible decision here is that the JVM gives you **neither**
concession the other two languages offer. `substring` copies, and has since JDK
7u6 removed the shared-array optimization, so there is no borrowed view to be
had. And `charAt` hands back a UTF-16 *code unit* rather than a character: a
non-BMP character is two of them, so a `charAt`-driven two-pointer walk gets
non-BMP input wrong in exactly the way a byte-level walk gets UTF-8 wrong in
Rust.

So this walks an array of **code points**, which on the JVM is an owned copy
either way. That is the honest summary of the trade across the three siblings:
Python pays a copy per slice, Rust can keep a borrow and allocate nothing, and
here you pay one copy up front to get correctness.

Code points are not the last word. A grapheme cluster is the unit a human would
call a character, and none of the three siblings reaches for it, so a combining
mark can be torn off its base character. The `code points are still not grapheme
clusters` test pins that limit rather than leaving it to chance.

## Run

```
sbt test       # 12 munit tests
sbt run        # tiny demo
```

## Files

- `src/main/scala/palindrome.scala` — `Palindrome.longestPalindrome(String): String` and an `expand` helper returning bounds
- `src/test/scala/PalindromeTests.scala` — munit suite, including non-BMP input and a randomized cross-check against brute force
- `build.sbt`, `project/build.properties` — sbt build (Scala 3.8.4, munit)
