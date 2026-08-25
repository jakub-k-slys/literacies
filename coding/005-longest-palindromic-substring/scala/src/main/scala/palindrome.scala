// Longest palindromic substring by expansion around centers, O(n^2).
//
// Coding Literacy - the idea is free, the notation is not.
//
// The expansion is the same walk as the Python and Rust siblings: 2n-1 centers,
// grow outward while the two ends match, keep the best bounds.
//
// The language-visible decision is that the JVM gives you neither concession.
// `substring` copies, and has since JDK 7u6 removed the shared-array
// optimization, so there is no borrowed view to be had. And `charAt` hands back
// a UTF-16 *code unit*, not a character: a non-BMP character is two of them, so
// a charAt-driven two-pointer walk gets non-BMP input wrong the same way a
// byte-level walk gets UTF-8 wrong in Rust.
//
// So this walks an array of code points. On the JVM that array is an owned copy
// either way, which is the honest summary of the trade: Python pays a copy per
// slice, Rust can keep a borrow, and here you pay a copy to get correctness.

object Palindrome:

  /** Expand outward from a center while both ends match, over code-point positions.
    *
    * Returns bounds, not a substring: both ends have overshot by one when the
    * loop exits, so the winning window is positions `l+1` until `r-1`, of
    * length `r - l - 1`.
    */
  private def expand(cps: Array[Int], l0: Int, r0: Int): (Int, Int) =
    var l = l0
    var r = r0
    while l >= 0 && r < cps.length && cps(l) == cps(r) do
      l -= 1
      r += 1
    (l, r)

  def longestPalindrome(s: String): String =
    val cps = s.codePoints().toArray // the copy the JVM makes you pay for correctness

    var bestStart = 0
    var bestLen = 0

    // Nothing is allocated per center: the two candidates are checked in place.
    inline def consider(bounds: (Int, Int)): Unit =
      val (l, r) = bounds
      val len = r - l - 1
      if len > bestLen then
        bestStart = l + 1
        bestLen = len

    var i = 0
    while i < cps.length do
      consider(expand(cps, i, i))     // odd center
      consider(expand(cps, i, i + 1)) // gap center
      i += 1

    new String(cps, bestStart, bestLen)

@main def demo(): Unit =
  // A tiny demo so `sbt run` does something visible.
  for sample <- List("babad", "cbbd", "forgeeksskeegfor") do
    println(s"longestPalindrome($sample) = ${Palindrome.longestPalindrome(sample)}")
