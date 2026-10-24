// Tests for ReverseInteger.reverse and ReverseInteger.reverseExact.
//
// Run: sbt test

class ReverseIntegerTests extends munit.FunSuite:
  import ReverseInteger.{reverse, reverseExact, reverseUnguarded}

  private val both = List[Int => Int](reverse, reverseExact)

  test("statement examples"):
    for f <- both do
      assertEquals(f(123), 321)
      assertEquals(f(-123), -321)
      assertEquals(f(120), 21) // 021 and 21 are the same integer

  test("zero"):
    for f <- both do assertEquals(f(0), 0)

  test("the reversal that needs more than 32 bits"):
    // 9646324351 does not fit; the guard refuses 964632435 * 10.
    for f <- both do assertEquals(f(1534236469), 0)

  test("minimum value"):
    for f <- both do assertEquals(f(Int.MinValue), 0) // -2147483648

  test("maximum value"):
    for f <- both do assertEquals(f(Int.MaxValue), 0) // 2147483647 -> 7463847412

  test("the only inputs that reach the bound"):
    for f <- both do
      assertEquals(f(1463847412), 2147483641)
      assertEquals(f(-1463847412), -2147483641)

  test("one past the bound is refused"):
    for f <- both do
      assertEquals(f(1563847412), 0) // 2147483651
      assertEquals(f(-1563847412), 0)
      assertEquals(f(1000000003), 0) // 3000000001

  test("trailing zeros and single digits"):
    for f <- both do
      assertEquals(f(-10), -1)
      assertEquals(f(1000000000), 1)
      assertEquals(f(7), 7)
      assertEquals(f(-7), -7)

  test("JVM int arithmetic wraps, in every build"):
    assertEquals(Int.MaxValue + 1, Int.MinValue)
    assertEquals(964632435 * 10, 1056389758) // the true product is 9646324350
    // So the unguarded loop returns a positive ten-digit number, not an error.
    assertEquals(reverseUnguarded(1534236469), 1056389759)

  test("abs of the minimum does not fit"):
    // Why neither version takes abs(x) first, as the Python one does.
    assertEquals(Math.abs(Int.MinValue), Int.MinValue)
    intercept[ArithmeticException](Math.absExact(Int.MinValue))

  test("truncating division keeps the sign in the digits"):
    // Python floors (-123 // 10 == -13, -123 % 10 == 7); the JVM truncates.
    assertEquals((-123 / 10, -123 % 10), (-12, -3))
    assertEquals((Math.floorDiv(-123, 10), Math.floorMod(-123, 10)), (-13, 7))
    assertEquals(Int.MinValue / 10, -214748364)

  test("agrees with a 64-bit oracle"):
    // The oracle widens to Long, which the solution is not allowed to do.
    def oracle(x: Int): Int =
      val r = x.toLong.abs.toString.reverse.toLong * x.sign
      if r.isValidInt then r.toInt else 0

    val rng = scala.util.Random(20261109)
    val samples = Seq.fill(200000)(rng.nextInt()) ++
      Seq(Int.MinValue, Int.MinValue + 1, -1, 0, 1, Int.MaxValue - 1, Int.MaxValue)
    for x <- samples do
      assertEquals(reverse(x), oracle(x), s"input $x")
      assertEquals(reverseExact(x), oracle(x), s"input $x")
