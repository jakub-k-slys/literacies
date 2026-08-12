# literacies

Runnable reference code behind the **slys.DEV** literacy series — the free code
artifact each post links to. One problem, one transferable idea, implemented the
same way in three languages so the *idea* stands out from the *language accident*.

Written up at [slys.DEV](https://slys.dev) · by [Jakub Slys](https://slys.dev).

## The three literacies

- **Coding Literacy (L1)** — read code and choose consciously. One problem, one
  idea, in Rust · Python · Scala. → [`coding/`](coding/)
- **Systems Literacy (L2)** and **AI Literacy (L3)** live in the write-ups; their
  code lands here as the series grows.

## Layout

Each problem is its own directory, `coding/<NNN-slug>/`, holding one small,
self-contained project per language:

```
coding/004-median-of-two-sorted-arrays/
  python/   uv project      — main.py + test_main.py        (uv run pytest)
  rust/     cargo project    — src/main.rs + #[cfg(test)]     (cargo test)
  scala/    sbt project      — src/main + src/test (munit)    (sbt test)
```

All three implement the same idea. Where they *differ* — sentinels, integer
widening to avoid overflow when averaging, the memory model — is the subject of
the paid **Language Literacy** deep dive on slys.DEV.

## Run everything

```
cd coding/<NNN-slug>/python && uv sync && uv run pytest
cd coding/<NNN-slug>/rust   && cargo test
cd coding/<NNN-slug>/scala  && sbt test
```

## Problems

| # | Problem | Idea | Post |
|---|---------|------|------|
| 004 | [Median of Two Sorted Arrays](coding/004-median-of-two-sorted-arrays/) | place a boundary, don't walk the data — `O(log min(m, n))` | _(link once published)_ |

## License

[MIT](LICENSE).
