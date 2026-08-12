//! Median of two sorted arrays in O(log min(m, n)).
//!
//! Coding Literacy - read the boundary, don't walk the data.
//!
//! Note the `i64` widening: inputs are `i32`, but `left_max + right_min` can
//! overflow `i32` when averaging, so boundary values are lifted to `i64` first.
//! That widening is a language-visible decision, not incidental.

pub fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    // Always binary-search the smaller array.
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let (m, n) = (a.len(), b.len());
    let left_size = (m + n + 1) / 2;

    let (mut lo, mut hi) = (0i64, m as i64);
    while lo <= hi {
        let i = (lo + hi) / 2; // cut in a
        let j = left_size as i64 - i; // cut in b, forced by i

        let l1 = if i == 0 { i64::MIN } else { a[(i - 1) as usize] as i64 };
        let r1 = if i == m as i64 { i64::MAX } else { a[i as usize] as i64 };
        let l2 = if j == 0 { i64::MIN } else { b[(j - 1) as usize] as i64 };
        let r2 = if j == n as i64 { i64::MAX } else { b[j as usize] as i64 };

        if l1 <= r2 && l2 <= r1 {
            return if (m + n) % 2 == 1 {
                l1.max(l2) as f64
            } else {
                (l1.max(l2) + r1.min(r2)) as f64 / 2.0
            };
        }
        if l1 > r2 {
            hi = i - 1;
        } else {
            lo = i + 1;
        }
    }
    panic!("inputs must be sorted and not both empty");
}

fn main() {
    // A tiny demo so `cargo run` does something visible.
    let (a, b) = ([1, 3, 8], [7, 9, 10, 11]);
    println!("median({a:?}, {b:?}) = {}", find_median_sorted_arrays(&a, &b));
}

#[cfg(test)]
mod tests {
    use super::find_median_sorted_arrays as med;

    #[test]
    fn odd_total() {
        assert_eq!(med(&[1, 3], &[2]), 2.0);
    }
    #[test]
    fn even_total() {
        assert_eq!(med(&[1, 2], &[3, 4]), 2.5);
    }
    #[test]
    fn one_array_empty() {
        assert_eq!(med(&[], &[1]), 1.0);
        assert_eq!(med(&[2, 4, 6], &[]), 4.0);
    }
    #[test]
    fn very_unbalanced() {
        assert_eq!(med(&[1], &[2, 3, 4, 5, 6]), 3.5);
    }
    #[test]
    fn duplicates_and_equal_boundaries() {
        assert_eq!(med(&[1, 1], &[1, 1]), 1.0);
        assert_eq!(med(&[1, 2, 2, 3], &[2, 2]), 2.0);
    }
    #[test]
    fn negative_and_mixed() {
        assert_eq!(med(&[-5, -3, -1], &[-2, 0, 2]), -1.5);
    }
}
