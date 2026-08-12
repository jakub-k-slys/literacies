# 004 · Median of Two Sorted Arrays — Python

`O(log min(m, n))` partition / binary-search solution.

## Idea

You do not *reach* the median by walking the merged data, you *place* a boundary.
Choose how many elements come from the smaller array's left side; that single
choice forces the cut in the other array. Binary-search the choice until the left
half is entirely `<=` the right half — then the median sits on the boundary.

Ends of a cut are handled with `+/- inf` sentinels, so an empty side never needs a
special case.

## Run

```
uv run pytest          # 6 tests
uv run main.py         # tiny demo
```

or without uv:

```
python -m pytest
python main.py
```

## Files

- `main.py` — `find_median_sorted_arrays(nums1, nums2) -> float`
- `test_main.py` — odd/even totals, empty side, unbalanced, duplicates, negatives
