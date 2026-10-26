# 008 · String to Integer (atoi) — Scala

One pass over the string, `O(n)` time, `O(1)` extra space, and a result that can
say **no** on a channel the value cannot occupy. An sbt project.

## Idea

The judge's atoi returns `0` for `"0"` and for `"words and 987"`, and
`Int.MaxValue` for `"2147483647"` and for `"99999999999"`. Every `Int` is
somebody's correct answer, so the *no* has to travel outside the value.

`Atoi.parseInt32Prefix(s: String): Prefix` is the core, with

```scala
enum Prefix:
  case Parsed(value: Int)
  case NoDigits
  case OutOfRange(negative: Boolean)
```

the same three named outcomes as the Python sibling, as a Scala 3 enum. A
`match` over it is checked for exhaustiveness: drop a case and the compiler
warns. `Atoi.myAtoi(s: String): Int` is the adapter the judge calls, and its
`NoDigits => 0` case is the only place that decides 0 means "nothing here".
Code that needs to know whether a zero was real calls the core, and can pass the
named case up the stack for someone higher up to pattern match on.

The grammar is fixed on purpose: the space character `' '` only
(`s.segmentLength(_ == ' ')`, not `trim`), one optional sign, digits
`'0' <= c <= '9'`. `Character.isDigit('٤')` is true and
`Character.digit('٤', 10)` is 4, so the ASCII test is what makes `"٤٢"`
`NoDigits`.

The sign stays in the digits. A negative number accumulates downward, and `rev`
is compared against `Int.MaxValue / 10` and `Int.MinValue / 10` (the digit
against `% 10` of each) before every push, as in Reverse Integer. So
`"-2147483648"` reaches `Int.MinValue` without a `Long`, the end the Python
sibling handles by briefly holding 2147483648. The check has to be in the code,
because the JVM wraps in two's complement without a word.

The standard parsers make a different trade. `"42".toIntOption` returns an
`Option`, which can say no but not why: `"4193 with words"`, `" 42"`, `"+"`, `""`
and `"99999999999"` are all `None`. `Integer.parseInt` says why only in its
`NumberFormatException` message. Both accept Unicode decimal digits:
`"٤٢".toIntOption` is `Some(42)`.

## Run

```
sbt test       # 12 munit tests
sbt run        # tiny demo
```

## Files

- `src/main/scala/atoi.scala` — `Atoi.Prefix`, `Atoi.parseInt32Prefix`, `Atoi.myAtoi`
- `src/test/scala/AtoiTests.scala` — munit suite: statement examples, the six inputs that all give 0, the core keeping them apart, clamping at both ends, the prefix rule, leading zeros, tab and NBSP not skipped, `"٤٢"` rejected, `toIntOption` / `Integer.parseInt` facts, randomized cross-check against a `Long` reference
- `build.sbt`, `project/build.properties` — sbt build (Scala 3.8.4, munit)
