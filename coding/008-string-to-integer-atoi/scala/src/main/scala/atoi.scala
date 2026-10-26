// String to Integer (atoi) as a parser that can say no.
//
// Coding Literacy - if zero is a valid answer, it cannot also be the error.
//
// Two layers, as in the Python and Rust siblings. `parseInt32Prefix` is the
// core and returns a `Prefix`, an enum of three named outcomes: Parsed(value),
// NoDigits, OutOfRange(negative). The no travels as a case, not as a value of
// Int, and a `match` over it is checked for exhaustiveness by the compiler.
// `myAtoi` is the adapter the judge calls, and the only place that decides 0
// means "nothing here".
//
// The grammar is fixed on purpose: the space character ' ' only (not trim or
// isWhitespace), one optional sign, digits '0' <= c <= '9' (not Character.isDigit,
// which is true for the Arabic-Indic '٤').
//
// The overflow guard is Reverse Integer's, with the sign kept in the digits. A
// negative number accumulates downward (rev * 10 - d), so Int.MinValue is
// reached without a Long, where the Python sibling briefly holds the magnitude
// 2147483648. rev is compared against Int.MaxValue / 10 and Int.MinValue / 10
// (and the digit against % 10 of each) before every push, because the JVM
// wraps in two's complement without a word.

import scala.annotation.tailrec

object Atoi:

  enum Prefix:
    case Parsed(value: Int)
    case NoDigits
    case OutOfRange(negative: Boolean)

  import Prefix.*

  private val MaxDiv = Int.MaxValue / 10 // 214748364
  private val MaxMod = Int.MaxValue % 10 // 7
  private val MinDiv = Int.MinValue / 10 // -214748364: truncating, so no off-by-one
  private val MinMod = Int.MinValue % 10 // -8

  /** Would rev * 10 + d leave Int? Decided using only values that fit. */
  private def overflows(rev: Int, d: Int): Boolean =
    rev > MaxDiv || (rev == MaxDiv && d > MaxMod) ||
      rev < MinDiv || (rev == MinDiv && d < MinMod)

  def parseInt32Prefix(s: String): Prefix =
    val afterSpaces = s.segmentLength(_ == ' ') // the space character only, not trim
    val sign = s.lift(afterSpaces)
    val negative = sign.contains('-')
    val start = if sign.exists(c => c == '+' || c == '-') then afterSpaces + 1 else afterSpaces

    @tailrec def digits(i: Int, rev: Int): Prefix =
      if i < s.length && '0' <= s(i) && s(i) <= '9' then
        val d = if negative then '0' - s(i) else s(i) - '0' // the sign stays in the digits
        if overflows(rev, d) then OutOfRange(negative) // refuse before the push
        else digits(i + 1, rev * 10 + d)
      else if i == start then NoDigits // the index never moved past the sign
      else Parsed(rev)

    digits(start, 0)

  def myAtoi(s: String): Int = parseInt32Prefix(s) match
    case Parsed(value)        => value
    case NoDigits             => 0 // the one place that decides 0 means "nothing here"
    case OutOfRange(negative) => if negative then Int.MinValue else Int.MaxValue

@main def demo(): Unit =
  // A tiny demo so `sbt run` does something visible.
  for sample <- List("42", "   -42", "4193 with words", "words and 987", "0", "99999999999") do
    val quoted = s"'$sample'"
    val core = Atoi.parseInt32Prefix(sample).toString
    println(f"$quoted%18s  parseInt32Prefix = $core%-18s  myAtoi = ${Atoi.myAtoi(sample)}")
