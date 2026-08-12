// Median of two sorted arrays in O(log min(m, n)).
//
// Coding Literacy - read the boundary, don't walk the data.
//
// Boundaries are lifted to Long before summing: on the JVM `Int` overflow is
// silent, so averaging two large Ints would wrap without a warning. Widening is
// the language-visible decision here.

object Median:
  def findMedianSortedArrays(nums1: Array[Int], nums2: Array[Int]): Double =
    val (a, b) = if nums1.length <= nums2.length then (nums1, nums2) else (nums2, nums1)
    val (m, n) = (a.length, b.length)
    if m + n == 0 then throw IllegalArgumentException("inputs must not both be empty")
    val leftSize = (m + n + 1) / 2

    var lo = 0
    var hi = m
    while lo <= hi do
      val i = (lo + hi) / 2 // cut in a
      val j = leftSize - i  // cut in b, forced by i

      val l1 = if i == 0 then Long.MinValue else a(i - 1).toLong
      val r1 = if i == m then Long.MaxValue else a(i).toLong
      val l2 = if j == 0 then Long.MinValue else b(j - 1).toLong
      val r2 = if j == n then Long.MaxValue else b(j).toLong

      if l1 <= r2 && l2 <= r1 then
        return
          if (m + n) % 2 == 1 then math.max(l1, l2).toDouble
          else (math.max(l1, l2) + math.min(r1, r2)).toDouble / 2.0
      else if l1 > r2 then hi = i - 1
      else lo = i + 1

    throw IllegalArgumentException("inputs must be sorted")

@main def demo(): Unit =
  val (a, b) = (Array(1, 3, 8), Array(7, 9, 10, 11))
  println(s"median(${a.mkString(",")}, ${b.mkString(",")}) = ${Median.findMedianSortedArrays(a, b)}")
