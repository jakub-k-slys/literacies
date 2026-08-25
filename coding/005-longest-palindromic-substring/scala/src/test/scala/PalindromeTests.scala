// Tests for Palindrome.longestPalindrome.
//
// Run: sbt test

class PalindromeTests extends munit.FunSuite:
  private def lps(s: String): String = Palindrome.longestPalindrome(s)

  test("statement examples"):
    // Ties are allowed, so the length is what is fixed, not the string.
    assert(Set("bab", "aba").contains(lps("babad")))
    assertEquals(lps("cbbd"), "bb")

  test("single character"):
    assertEquals(lps("a"), "a")

  test("empty"):
    assertEquals(lps(""), "")

  test("no palindrome longer than one"):
    assertEquals(lps("abcde").length, 1)

  test("even length needs the gap centers"):
    // Drop the gap centers and these return a single character instead.
    assertEquals(lps("abba"), "abba")
    assertEquals(lps("bb"), "bb")

  test("whole string is the answer"):
    assertEquals(lps("racecar"), "racecar")
    assertEquals(lps("aaaa"), "aaaa")

  test("answer is interior"):
    assertEquals(lps("forgeeksskeegfor"), "geeksskeeg")

  test("off by one at the edges"):
    assertEquals(lps("aab"), "aa")
    assertEquals(lps("baa"), "aa")

  test("non-BMP characters are one unit each"):
    // U+1F600 is a single character stored as two UTF-16 code units. A
    // charAt-driven two-pointer walk compares surrogates and reports length 1;
    // walking code points gets the whole string.
    val emoji = "😀"
    val s = emoji + "a" + emoji
    assertEquals(s.length, 5)             // UTF-16 code units
    assertEquals(s.codePointCount(0, s.length), 3) // characters
    assertEquals(lps(s), s)

  test("surrogate pair is never split"):
    // The answer must stay well-formed text, never half of a character.
    val s = "😀😁"
    val got = lps(s)
    assertEquals(got.codePointCount(0, got.length), 1)
    assert(!Character.isHighSurrogate(got.charAt(got.length - 1)))

  test("agrees with brute force"):
    // O(n^3) oracle: every window, reversed and compared. StringOps.reverse
    // delegates to StringBuilder.reverse, which the JVM documents as keeping
    // surrogate pairs intact, so it is a safe oracle here.
    def brute(s: String): Int =
      var best = 0
      for
        i <- 0 to s.length
        j <- i + 1 to s.length
      do
        val w = s.substring(i, j)
        if w == w.reverse then best = math.max(best, w.length)
      best

    val rng = scala.util.Random(20260825)
    for _ <- 0 until 500 do
      val n = rng.nextInt(15)
      val s = List.fill(n)("abc".charAt(rng.nextInt(3))).mkString
      val got = lps(s)
      assertEquals(got.length, brute(s), s"input '$s'")
      assertEquals(got, got.reverse, s"input '$s'")
      assert(s.contains(got), s"input '$s'")
