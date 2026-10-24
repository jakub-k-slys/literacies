# 007 · Reverse Integer — Scala

One pass over the digits (at most 10 for an `Int`), `O(1)` extra state, the
32-bit boundary checked before every push. An sbt project.

## Idea

Pop the last digit, drop it, push it onto the result. The push is the only step
that can leave the 32-bit range, and the problem's answer for that case is 0.

The language-visible decision here is that **the JVM never tells you**. Scala's
`Int` is the JVM's `int`, and the JVM has no debug mode for arithmetic: `+` and
`*` wrap in two's complement in every build. `Int.MaxValue + 1 == Int.MinValue`,
and `964632435 * 10 == 1056389758`, the value the unguarded loop
(`reverseUnguarded`) is holding partway through `1534236469`. It returns
`1056389759`: positive, ten digits, and wrong. The debug/release split is Rust's,
not Scala's.

So the check has to be in the code, and there are two ways to write it:

- `reverse` checks by hand, **before** the multiply: `rev` against
  `Int.MaxValue / 10` and `Int.MinValue / 10`, the digit against `% 10` of each
  (7 on the positive side, -8 on the negative). No value outside `Int` is ever
  formed. This is the honest solution under the problem's no-64-bit rule.
- `reverseExact` asks the library: `Math.multiplyExact` and `Math.addExact`
  throw an `ArithmeticException` instead of wrapping, and the catch turns it into
  0. It is the convenient one. In the OpenJDK source, `multiplyExact(int, int)`
  widens to `long` internally before it checks, so strictly it is a 64-bit
  operation; `addExact` is not (it tests sign bits).

The sign stays in the digits. The JVM's `/` and `%` truncate toward zero, so a
negative `x` yields negative digits and `rev` grows toward `Int.MinValue` by
itself. The Python sibling's `abs(x)`-first move is broken here:
`Math.abs(Int.MinValue) == Int.MinValue`, and `Math.absExact` throws.

## Run

```
sbt test       # 12 munit tests
sbt run        # tiny demo
```

## Files

- `src/main/scala/reverse.scala` — `ReverseInteger.reverse` (hand-written pre-check), `ReverseInteger.reverseExact` (`Math.*Exact`), `ReverseInteger.reverseUnguarded` (the wrapping loop)
- `src/test/scala/ReverseIntegerTests.scala` — munit suite: statement examples, `Int.MinValue`, `Int.MaxValue`, `1534236469`, the bound inputs `±1463847412`, JVM wrapping and `abs` facts, truncating division, randomized cross-check against a `Long` oracle
- `build.sbt`, `project/build.properties` — sbt build (Scala 3.8.4, munit)
