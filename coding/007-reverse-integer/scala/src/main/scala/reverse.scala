// Reverse Integer with the 32-bit boundary checked before the push.
//
// Coding Literacy - overflow is a language decision.
//
// The loop is the same three steps as the Python and Rust siblings: pop the
// last digit, drop it, push it onto the result. What happens on the push that
// crosses 2^31 is not in the algorithm. Scala's Int is the JVM's int, and the
// JVM has no debug mode for arithmetic: + and * wrap in two's complement in
// every build, without a word. So the check has to be in the code.
//
// Two versions. `reverse` checks by hand, before the multiply, comparing rev
// against Int.MaxValue / 10 and Int.MinValue / 10 (and the last digit against
// the % 10 of each), so no value outside Int is ever formed. `reverseExact`
// asks the library: Math.multiplyExact and Math.addExact throw an
// ArithmeticException instead of wrapping. It is the convenient one, though
// multiplyExact(int, int) widens to long internally before it checks, so under
// the problem's strict no-64-bit rule the hand-written check is the honest one.
//
// The sign stays in the digits. The JVM's / and % truncate toward zero, so a
// negative x yields negative digits. The Python sibling's abs(x)-first move is
// broken here: Math.abs(Int.MinValue) == Int.MinValue.

import scala.annotation.tailrec

object ReverseInteger:

  private val MaxDiv = Int.MaxValue / 10 // 214748364
  private val MaxMod = Int.MaxValue % 10 // 7
  private val MinDiv = Int.MinValue / 10 // -214748364: truncating, so no off-by-one
  private val MinMod = Int.MinValue % 10 // -8

  /** Would rev * 10 + d leave Int? Decided using only values that fit. */
  private def overflows(rev: Int, d: Int): Boolean =
    rev > MaxDiv || (rev == MaxDiv && d > MaxMod) ||
      rev < MinDiv || (rev == MinDiv && d < MinMod)

  def reverse(x: Int): Int =
    @tailrec def go(x: Int, rev: Int): Int =
      if x == 0 then rev
      else
        val d = x % 10 // truncating: d carries x's sign, so no Math.abs and no Int.MinValue trap
        if overflows(rev, d) then 0 // refuse the push before making it
        else go(x / 10, rev * 10 + d)
    go(x, 0)

  def reverseExact(x: Int): Int =
    @tailrec def go(x: Int, rev: Int): Int =
      if x == 0 then rev
      else go(x / 10, Math.addExact(Math.multiplyExact(rev, 10), x % 10))
    try go(x, 0)
    catch case _: ArithmeticException => 0 // the boundary, reported as an exception

  /** The same loop with the guard missing: on the JVM it wraps, in every build. */
  def reverseUnguarded(x: Int): Int =
    @tailrec def go(x: Int, rev: Int): Int =
      if x == 0 then rev else go(x / 10, rev * 10 + x % 10)
    go(x, 0)

@main def demo(): Unit =
  // A tiny demo so `sbt run` does something visible.
  for sample <- List(123, -123, 120, 1534236469, Int.MinValue) do
    println(s"reverse($sample) = ${ReverseInteger.reverse(sample)}")
