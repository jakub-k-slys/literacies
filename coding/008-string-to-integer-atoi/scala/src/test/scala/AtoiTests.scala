// Tests for Atoi.parseInt32Prefix and Atoi.myAtoi.
//
// Run: sbt test

class AtoiTests extends munit.FunSuite:
  import Atoi.{myAtoi, parseInt32Prefix}
  import Atoi.Prefix.*

  private val Nbsp = "\u00a0" // a non-breaking space, kept as an escape on purpose
  private val Arabic42 = "\u0664\u0662" // "٤٢", Arabic-Indic four and two

  test("statement examples"):
    assertEquals(myAtoi("42"), 42)
    assertEquals(myAtoi(" -042"), -42)
    assertEquals(myAtoi("1337c0d3"), 1337)
    assertEquals(myAtoi("0-1"), 0)
    assertEquals(myAtoi("words and 987"), 0)
    assertEquals(myAtoi("-91283472332"), Int.MinValue) // constructed: below Int.MinValue, clamps

  test("earlier statement examples"):
    assertEquals(myAtoi("   -42"), -42)
    assertEquals(myAtoi("4193 with words"), 4193)
    assertEquals(myAtoi("0"), 0)

  test("zero and no digits are one answer from atoi"):
    for s <- List("0", "-0", "", "   ", "+", "abc") do assertEquals(myAtoi(s), 0, s"input '$s'")

  test("the core keeps them apart"):
    assertEquals(parseInt32Prefix("0"), Parsed(0))
    assertEquals(parseInt32Prefix("-0"), Parsed(0))
    for s <- List("words and 987", "abc", "", "   ", "+", "-", "+-12") do
      assertEquals(parseInt32Prefix(s), NoDigits, s"input '$s'")

  test("the clamp is also two answers from atoi"):
    assertEquals(myAtoi("2147483647"), Int.MaxValue)
    assertEquals(myAtoi("99999999999"), Int.MaxValue)
    assertEquals(parseInt32Prefix("2147483647"), Parsed(Int.MaxValue))
    assertEquals(parseInt32Prefix("99999999999"), OutOfRange(negative = false))

  test("clamping both ends"):
    assertEquals(myAtoi("2147483648"), Int.MaxValue)
    assertEquals(myAtoi("-2147483649"), Int.MinValue)
    assertEquals(myAtoi("-2147483648"), Int.MinValue)
    // Reached by accumulating downward; no Long and no Math.abs anywhere.
    assertEquals(parseInt32Prefix("-2147483648"), Parsed(Int.MinValue))
    assertEquals(parseInt32Prefix("2147483648"), OutOfRange(negative = false))
    assertEquals(parseInt32Prefix("-2147483649"), OutOfRange(negative = true))

  test("the prefix rule"):
    assertEquals(parseInt32Prefix("1337c0d3"), Parsed(1337))
    assertEquals(parseInt32Prefix("0-1"), Parsed(0))
    assertEquals(parseInt32Prefix(" -042"), Parsed(-42))
    assertEquals(parseInt32Prefix("30s"), Parsed(30))
    assertEquals(myAtoi(" +0 123"), 0)
    assertEquals(parseInt32Prefix(" +0 123"), Parsed(0))
    assertEquals(myAtoi("30s"), 30) // the unit disappears without a word
    assertEquals(myAtoi("1-2"), 1)

  test("leading zeros do not count toward the range"):
    assertEquals(myAtoi("0000000000012345678"), 12345678)
    assertEquals(myAtoi("-000000000002147483648"), Int.MinValue)

  test("only the space character is skipped"):
    assertEquals(parseInt32Prefix("\t42"), NoDigits)
    assertEquals(parseInt32Prefix(s"${Nbsp}42"), NoDigits)
    assertEquals(myAtoi("\n42"), 0)
    // Java's String.strip() would have skipped the tab, though not the NBSP.
    assertEquals("\t42".strip(), "42")
    assert(!Character.isWhitespace('\u00a0'))

  test("only ASCII digits count"):
    assertEquals(parseInt32Prefix(Arabic42), NoDigits)
    assertEquals(myAtoi(Arabic42), 0)
    assertEquals(parseInt32Prefix("4\u00b2"), Parsed(4)) // "4²": the superscript ends the prefix
    assert(Character.isDigit('\u0664'))
    assertEquals(Character.digit('\u0664', 10), 4)

  test("the standard parsers say no, and only Java's says why"):
    // The whole string or nothing: no prefix rule, no space skipping.
    for s <- List("1337c0d3", "4193 with words", " 42", "+", "", "99999999999") do
      assertEquals(s.toIntOption, None, s"input '$s'")
    intercept[NumberFormatException](Integer.parseInt("1337c0d3"))
    // Integer.parseInt says why, in the exception's message text.
    val e = intercept[NumberFormatException](Integer.parseInt("99999999999"))
    assert(e.getMessage.contains("99999999999"))
    // Both accept Unicode decimal digits, as Python's int() does (Rust's parse does not).
    assertEquals(Integer.parseInt(Arabic42), 42)
    assertEquals(Arabic42.toIntOption, Some(42))

  test("agrees with a 64-bit reference on ASCII inputs"):
    // The reference widens to Long and clamps, which the solution never does.
    def reference(s: String): Int =
      val m = raw" *([+-]?[0-9]+).*".r
      s match
        case m(num) =>
          val digits = num.dropWhile(c => c == '+' || c == '-').dropWhile(_ == '0')
          val v = if digits.length > 11 then Long.MaxValue else if digits.isEmpty then 0L else digits.toLong
          val signed = if num.startsWith("-") then -v else v
          signed.max(Int.MinValue.toLong).min(Int.MaxValue.toLong).toInt
        case _ => 0

    val rng = scala.util.Random(20261116)
    val alphabet = " +-0123456789a"
    for _ <- 1 to 100000 do
      val s = Seq.fill(rng.nextInt(15))(alphabet(rng.nextInt(alphabet.length))).mkString
      assertEquals(myAtoi(s), reference(s), s"input '$s'")
