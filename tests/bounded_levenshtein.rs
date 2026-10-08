use strsim::{bounded_levenshtein, levenshtein};

#[test]
fn threshold_boundaries_and_unicode() {
    let cases = [
        ("", "", 0),
        ("", "🦀中", 2),
        ("kitten", "sitting", 3),
        ("same", "same", 0),
        ("öঙ香", "abc", 3),
        ("🦀x中", "🦀中", 1),
        ("é", "e\u{301}", 2),
        ("abababab", "babababa", 2),
        ("abc", "xabcx", 2),
        ("abc", "xxxxxxxx", 8),
    ];
    for &(a, b, distance) in &cases {
        for &(left, right) in &[(a, b), (b, a)] {
            assert_eq!(Some(distance), bounded_levenshtein(left, right, distance));
            assert_eq!(Some(distance), bounded_levenshtein(left, right, usize::MAX));
            if distance > 0 {
                assert_eq!(None, bounded_levenshtein(left, right, distance - 1));
            }
        }
    }
}

#[test]
fn matches_unbounded_distance_for_small_strings() {
    let mut words = vec![String::new()];
    for len in 1..=5 {
        for bits in 0..1usize << len {
            words.push(
                (0..len)
                    .map(|j| if bits & (1 << j) == 0 { 'a' } else { 'é' })
                    .collect(),
            );
        }
    }
    for word in &["🦀x中", "🦀中", "e\u{301}", "é", ""] {
        words.push((*word).to_owned());
    }

    for a in &words {
        for b in &words {
            let distance = levenshtein(a, b);
            for limit in 0..=6 {
                let expected = if distance <= limit {
                    Some(distance)
                } else {
                    None
                };
                assert_eq!(
                    expected,
                    bounded_levenshtein(a, b, limit),
                    "a={:?}, b={:?}, limit={}",
                    a,
                    b,
                    limit
                );
            }
            assert_eq!(Some(distance), bounded_levenshtein(a, b, usize::MAX));
        }
    }
}
