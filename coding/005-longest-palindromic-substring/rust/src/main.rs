//! Longest palindromic substring by expansion around centers, O(n^2).
//!
//! Coding Literacy - the idea is free, the notation is not.
//!
//! The expansion is the same walk as the Python and Scala siblings: 2n-1 centers,
//! grow outward while the two ends match, keep the best bounds.
//!
//! The language-visible decision is the **unit**. A `&str` is a borrowed view
//! into UTF-8 bytes, so `&s[i..j]` copies nothing - but `i` counts bytes, not
//! characters. Walking bytes does not merely run slower on non-ASCII input, it
//! returns the wrong answer, and then panics when the winning range lands inside
//! a character (`&"aóòa"[1..4]` -> "byte index 4 is not a char boundary").
//!
//! So this walks a `char_indices` table instead. That costs one index walk and
//! keeps the borrow: the return type is `&str`, pointing straight into the
//! caller's buffer, and no `String` is ever built. Collecting a `Vec<char>`
//! would also be correct and would give the borrow up, since `Vec<char>` is an
//! owned copy at four bytes per character.

/// Expand outward from a center while both ends match, over character positions.
///
/// Returns bounds, not a slice: both ends have overshot by one when the loop
/// exits, so the winning window is characters `l+1 ..= r-1`, of length `r-l-1`.
fn expand(cs: &[(usize, char)], mut l: isize, mut r: isize) -> (isize, isize) {
    while l >= 0 && (r as usize) < cs.len() && cs[l as usize].1 == cs[r as usize].1 {
        l -= 1;
        r += 1;
    }
    (l, r)
}

pub fn longest_palindrome(s: &str) -> &str {
    // The one index walk. Each entry is (byte offset, character).
    let cs: Vec<(usize, char)> = s.char_indices().collect();

    let (mut best_start, mut best_len) = (0usize, 0usize); // in characters
    for i in 0..cs.len() {
        let i = i as isize;
        for (l, r) in [expand(&cs, i, i), expand(&cs, i, i + 1)] {
            // odd center, then gap center
            let len = (r - l - 1) as usize;
            if len > best_len {
                best_start = (l + 1) as usize;
                best_len = len;
            }
        }
    }

    if best_len == 0 {
        return "";
    }
    // Character bounds back to byte bounds, then one borrowed slice.
    let start = cs[best_start].0;
    let (last_offset, last_char) = cs[best_start + best_len - 1];
    &s[start..last_offset + last_char.len_utf8()]
}

fn main() {
    // A tiny demo so `cargo run` does something visible.
    for sample in ["babad", "cbbd", "forgeeksskeegfor"] {
        println!("longest_palindrome({sample:?}) = {:?}", longest_palindrome(sample));
    }
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome as lps;

    #[test]
    fn statement_examples() {
        // Ties are allowed, so the length is what is fixed, not the string.
        assert!(matches!(lps("babad"), "bab" | "aba"));
        assert_eq!(lps("cbbd"), "bb");
    }

    #[test]
    fn single_character() {
        assert_eq!(lps("a"), "a");
    }

    #[test]
    fn empty() {
        assert_eq!(lps(""), "");
    }

    #[test]
    fn no_palindrome_longer_than_one() {
        assert_eq!(lps("abcde").chars().count(), 1);
    }

    #[test]
    fn even_length_needs_the_gap_centers() {
        // Drop the gap centers and these return a single character instead.
        assert_eq!(lps("abba"), "abba");
        assert_eq!(lps("bb"), "bb");
    }

    #[test]
    fn whole_string_is_the_answer() {
        assert_eq!(lps("racecar"), "racecar");
        assert_eq!(lps("aaaa"), "aaaa");
    }

    #[test]
    fn answer_is_interior() {
        assert_eq!(lps("forgeeksskeegfor"), "geeksskeeg");
    }

    #[test]
    fn off_by_one_at_the_edges() {
        assert_eq!(lps("aab"), "aa");
        assert_eq!(lps("baa"), "aa");
    }

    #[test]
    fn multibyte_characters_are_one_unit_each() {
        // "óaó" is a palindrome by characters. Its UTF-8 is C3 B3 61 C3 B3, whose
        // byte reversal is B3 C3 61 B3 C3, so a byte-level walk reports length 1
        // and misses this entirely.
        assert_eq!(lps("óaó"), "óaó");
        assert_eq!(lps("óò"), "ó");
    }

    #[test]
    fn the_input_that_makes_a_byte_walk_panic() {
        // "aóòa" is 61 C3 B3 C3 B2 61. A byte-level expand-around-center picks
        // bytes 1..4 as its best range, and &s[1..4] panics because byte 4 sits
        // inside 'ò'. Walking characters, the answer is a single character and
        // nothing panics.
        let s = "aóòa";
        assert_eq!(s.len(), 6); // bytes
        assert_eq!(s.chars().count(), 4); // characters
        assert!(!s.is_char_boundary(4));
        assert_eq!(lps(s).chars().count(), 1);
    }

    #[test]
    fn the_result_borrows_the_input_and_never_copies() {
        // The point of the char_indices route: no String is built, so the answer
        // is a pointer into the caller's own buffer.
        let s = String::from("zqracecarwv"); // wrapper is deliberately not symmetric
        let got = lps(&s);
        assert_eq!(got, "racecar");
        let base = s.as_ptr() as usize;
        let got_ptr = got.as_ptr() as usize;
        assert!(got_ptr >= base && got_ptr < base + s.len());
        assert_eq!(got_ptr - base, 2); // exactly where "racecar" starts in s
    }

    #[test]
    fn agrees_with_brute_force() {
        // O(n^3) oracle: every window, reversed and compared.
        fn brute(s: &str) -> usize {
            let cs: Vec<char> = s.chars().collect();
            let mut best = 0;
            for i in 0..cs.len() {
                for j in i + 1..=cs.len() {
                    let w = &cs[i..j];
                    if w.iter().eq(w.iter().rev()) {
                        best = best.max(w.len());
                    }
                }
            }
            best
        }

        // A tiny deterministic LCG keeps this dependency-free.
        let mut state: u64 = 20260825;
        let alphabet = ['a', 'b', 'c'];
        for _ in 0..500 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let n = (state >> 33) as usize % 15;
            let s: String = (0..n)
                .map(|_| {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    alphabet[(state >> 33) as usize % alphabet.len()]
                })
                .collect();
            let got = lps(&s);
            assert_eq!(got.chars().count(), brute(&s), "input {s:?}");
            assert!(got.chars().eq(got.chars().rev()), "input {s:?}");
            assert!(s.contains(got), "input {s:?}");
        }
    }
}
