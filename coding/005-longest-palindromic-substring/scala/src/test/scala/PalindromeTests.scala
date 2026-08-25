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

  test("code points are still not grapheme clusters"):
    // The limit all three siblings share, pinned rather than left to chance. An
    // accented e written decomposed is e + U+0301, so two of them are the four
    // code points e U+0301 e U+0301. By code points the longest palindrome is
    // three long and tears a combining mark off its base character. Two windows
    // tie at three, so both are pinned below.
    val acute = 0x0301
    val decomposed = String(Array('e'.toInt, acute, 'e'.toInt, acute), 0, 4)
    val got = lps(decomposed)
    assertEquals(got.codePointCount(0, got.length), 3)
    // Pin the two ties explicitly. Either way a combining mark has been torn off
    // its base character, which a 3-code-point window of this string must do.
    val tieA = String(Array('e'.toInt, acute, 'e'.toInt), 0, 3)
    val tieB = String(Array(acute, 'e'.toInt, acute), 0, 3)
    assert(got == tieA || got == tieB, s"unexpected tie: ${got.codePoints().toArray.mkString(",")}")

  test("agrees with brute force"):
    // The oracle walks CODE POINTS, not UTF-16 units. Indexing the string with
    // substring would let a window split a surrogate pair, and the oracle itself
    // would stop being sound on exactly the input this project is about.
    def brute(cps: Array[Int]): Int =
      var best = 0
      for
        i <- 0 to cps.length
        j <- i + 1 to cps.length
      do
        val w = cps.slice(i, j)
        if w.sameElements(w.reverse) then best = math.max(best, w.length)
      best

    // Non-BMP characters included on purpose: a surrogate pair is the whole
    // reason this implementation walks code points at all.
    val alphabet = Array('a'.toInt, 'b'.toInt, 0x00F3, 0x4E2D, 0x1F600, 0x1F601)
    val rng = scala.util.Random(20260825)
    for _ <- 0 until 2000 do
      val n = rng.nextInt(15)
      val cps = Array.fill(n)(alphabet(rng.nextInt(alphabet.length)))
      val s = String(cps, 0, cps.length)
      val got = lps(s)
      val gotCps = got.codePoints().toArray
      assertEquals(gotCps.length, brute(cps), s"input ${cps.mkString(",")}")
      assert(gotCps.sameElements(gotCps.reverse), s"input ${cps.mkString(",")}")
      assert(s.contains(got), s"input ${cps.mkString(",")}")
