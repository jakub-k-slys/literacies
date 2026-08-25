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
//! Rust lets you have both, if you step the string by character instead of
//! indexing it. `char_indices` yields byte offsets that are already character
//! boundaries, and `chars().next_back()` / `chars().next()` step one character
//! out from a boundary in constant time, because UTF-8 is self-synchronizing.
//! So this walk carries byte offsets throughout, allocates nothing at all, and
//! returns a `&str` pointing straight into the caller's buffer.
//!
//! The obvious alternative, collecting a `Vec<char>`, is also correct and gives
//! the borrow up: it throws the byte offsets away, so the answer has to be
//! rebuilt as an owned `String`.

/// Step outward from a window while the characters on either side match.
///
/// `lo` and `hi` are byte offsets, and both always sit on character boundaries.
/// `n` is how many characters the starting window already holds. Returns the
/// widest palindromic window around the same center: byte bounds plus its length
/// **in characters**, which is the length the problem actually asks about. Byte
/// length would rank a single four-byte character above a genuine two-character
/// palindrome.
fn expand(s: &str, mut lo: usize, mut hi: usize, mut n: usize) -> (usize, usize, usize) {
    loop {
        // Both are O(1): UTF-8 is self-synchronizing, so stepping one character
        // out from a boundary reads at most four bytes.
        let (Some(l), Some(r)) = (s[..lo].chars().next_back(), s[hi..].chars().next()) else {
            return (lo, hi, n);
        };
        if l != r {
            return (lo, hi, n);
        }
        lo -= l.len_utf8();
        hi += r.len_utf8();
        n += 2;
    }
}

pub fn longest_palindrome(s: &str) -> &str {
    let (mut best_lo, mut best_hi, mut best_n) = (0usize, 0usize, 0usize);

    for (i, c) in s.char_indices() {
        let after = i + c.len_utf8();
        // The odd center is the character itself, one character wide; the gap
        // center is the empty window just after it. Together, over every i, that
        // is the 2n-1 anchors.
        for (lo, hi, n) in [expand(s, i, after, 1), expand(s, after, after, 0)] {
            if n > best_n {
                (best_lo, best_hi, best_n) = (lo, hi, n);
            }
        }
    }

    &s[best_lo..best_hi] // borrowed, not built: the empty case borrows too
}

fn main() {
    // A tiny demo so `cargo run` does something visible.
    for sample in ["babad", "cbbd", "forgeeksskeegfor"] {
        println!("longest_palindrome({sample:?}) = {:?}", longest_palindrome(sample));
    }
}

#[cfg(test)]
mod tests {
    use super::{expand, longest_palindrome as lps};

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
    fn expand_returns_bounds_not_a_slice() {
        // The contract the whole design rests on: the helper hands back byte
        // bounds, so nothing is materialized per anchor.
        assert_eq!(expand("aba", 1, 2, 1), (0, 3, 3)); // odd center on 'b', grows both ways
        assert_eq!(expand("ab", 1, 1, 0), (1, 1, 0)); // gap center, immediate mismatch
        assert_eq!(expand("aa", 1, 1, 0), (0, 2, 2)); // gap center, grows to the whole string

        // The character count is not the byte count: this window is 2 characters
        // and 8 bytes, and ranking by bytes is what the fuzzer caught.
        assert_eq!(expand("😀😀", 4, 4, 0), (0, 8, 2));
    }

    #[test]
    fn multibyte_characters_are_one_unit_each() {
        // "óaó" is a palindrome by characters but not by bytes: its UTF-8 is
        // C3 B3 61 C3 B3, whose byte reversal is B3 C3 61 B3 C3. A byte-level
        // walk reports length 1 and misses this entirely.
        assert_eq!(lps("óaó"), "óaó");
        assert_eq!(lps("óò"), "ó");
    }

    #[test]
    fn the_input_that_makes_a_byte_walk_panic() {
        // "aóòa" is 61 C3 B3 C3 B2 61. A byte-level expand-around-center picks
        // bytes 1..4 as its best range, and &s[1..4] panics because byte 4 sits
        // inside 'ò'. Stepping by character, the answer is one character and
        // nothing panics.
        let s = "aóòa";
        assert_eq!(s.len(), 6); // bytes
        assert_eq!(s.chars().count(), 4); // characters
        assert!(!s.is_char_boundary(4));
        assert_eq!(lps(s).chars().count(), 1);
    }

    #[test]
    fn code_points_are_still_not_grapheme_clusters() {
        // The limit all three siblings share, pinned rather than left to chance.
        // "éé" written in decomposed form is e U+0301 e U+0301: by code points the
        // longest palindrome is U+0301 e U+0301, which splits a combining mark
        // away from its base. Grapheme clusters are the next unit up, and none of
        // the three implementations reaches for them.
        // Two windows tie at three code points here, so pin the length rather
        // than which tie wins; the Python and Scala siblings assert the same.
        let decomposed = "e\u{0301}e\u{0301}";
        let got = lps(decomposed);
        assert_eq!(got.chars().count(), 3);
        // Whichever tie wins, it has torn a combining mark off its base character.
        assert!(got.starts_with('\u{0301}') || !got.ends_with('\u{0301}'));
    }

    #[test]
    fn the_result_borrows_the_input_and_never_copies() {
        // The point of stepping by character rather than collecting: no owned
        // buffer of any kind, so the answer is a pointer into the caller's string.
        let s = String::from("zqracecarwv"); // wrapper is deliberately not symmetric
        let got = lps(&s);
        assert_eq!(got, "racecar");
        let base = s.as_ptr() as usize;
        assert_eq!(got.as_ptr() as usize - base, 2); // exactly where "racecar" starts

        // Even the empty answer is a slice of the input, not a 'static literal.
        let empty = String::new();
        assert_eq!(lps(&empty).as_ptr(), empty.as_ptr());
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

        // A tiny deterministic LCG keeps this dependency-free. The alphabet mixes
        // 1, 2 and 4-byte characters so the byte/character split is exercised.
        let mut state: u64 = 20260825;
        let mut next = move || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (state >> 33) as usize
        };
        let alphabet = ['a', 'b', 'ó', 'ò', '😀'];
        for _ in 0..2000 {
            let n = next() % 15;
            let s: String = (0..n).map(|_| alphabet[next() % alphabet.len()]).collect();
            let got = lps(&s);
            assert_eq!(got.chars().count(), brute(&s), "input {s:?}");
            assert!(got.chars().eq(got.chars().rev()), "input {s:?}");
            assert!(s.contains(got), "input {s:?}");
        }
    }
}
