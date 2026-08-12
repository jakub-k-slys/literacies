// Tests for Median.findMedianSortedArrays.
//
// Run: sbt test

class MedianTests extends munit.FunSuite:
  private def med(a: Array[Int], b: Array[Int]): Double =
    Median.findMedianSortedArrays(a, b)

  test("odd total"):
    assertEquals(med(Array(1, 3), Array(2)), 2.0)

  test("even total"):
    assertEquals(med(Array(1, 2), Array(3, 4)), 2.5)

  test("one array empty"):
    assertEquals(med(Array(), Array(1)), 1.0)
    assertEquals(med(Array(2, 4, 6), Array()), 4.0)

  test("very unbalanced"):
    assertEquals(med(Array(1), Array(2, 3, 4, 5, 6)), 3.5)

  test("duplicates and equal boundaries"):
    assertEquals(med(Array(1, 1), Array(1, 1)), 1.0)
    assertEquals(med(Array(1, 2, 2, 3), Array(2, 2)), 2.0)

  test("negative and mixed"):
    assertEquals(med(Array(-5, -3, -1), Array(-2, 0, 2)), -1.5)
