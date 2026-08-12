from typing import List

def find_median_sorted_arrays(nums1: List[int], nums2: List[int]) -> float:
    # Always binary-search the smaller array: O(log min(m, n)).
    a, b = (nums1, nums2) if len(nums1) <= len(nums2) else (nums2, nums1)
    m, n = len(a), len(b)
    left_size = (m + n + 1) // 2  # left half gets the extra element when total is odd

    lo, hi = 0, m
    while lo <= hi:
        i = (lo + hi) // 2       # cut in a
        j = left_size - i        # cut in b, forced by i

        # Boundary values around the cuts; +/- inf when a cut sits at an end.
        l1 = float("-inf") if i == 0 else a[i - 1]
        r1 = float("inf") if i == m else a[i]
        l2 = float("-inf") if j == 0 else b[j - 1]
        r2 = float("inf") if j == n else b[j]

        if l1 <= r2 and l2 <= r1:            # valid partition
            if (m + n) % 2 == 1:
                return float(max(l1, l2))
            return (max(l1, l2) + min(r1, r2)) / 2.0
        if l1 > r2:
            hi = i - 1                        # took too many from a
        else:
            lo = i + 1                        # took too few from a

    raise ValueError("inputs must be sorted and not both empty")


def test_find_median_sorted_arrays():
    a = [1, 3, 8]
    b = [7, 9, 10, 11]
    assert find_median_sorted_arrays(a, b) != 8

def main():
    print("Hello from 004-median-of-two-sorted-arrays!")


if __name__ == "__main__":
    main()
