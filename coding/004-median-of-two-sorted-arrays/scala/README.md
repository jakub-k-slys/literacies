# 004 · Median of Two Sorted Arrays — Scala

`O(log min(m, n))` partition / binary-search solution. An sbt project.

## Idea

You do not *reach* the median by walking the merged data, you *place* a boundary.
Choose how many elements come from the smaller array's left side; that single
choice forces the cut in the other array. Binary-search the choice until the left
half is entirely `<=` the right half.

The language-visible decision here is **`Long` widening**: on the JVM `Int`
overflow is *silent* (it wraps, no exception), so summing two boundary values to
average them is lifted to `Long` first. The `+/- inf` sentinels become
`Long.MinValue` / `Long.MaxValue`.

## Run

```
sbt test       # 6 munit tests
sbt run        # tiny demo
```

## Files

- `src/main/scala/median.scala` — `Median.findMedianSortedArrays(Array[Int], Array[Int]): Double`
- `src/test/scala/MedianTests.scala` — munit suite
- `build.sbt`, `project/build.properties` — sbt build (Scala 3.8.4, munit)
