//! Benchmarks for strsim.

#![feature(test)]

extern crate strsim;
extern crate test;
use self::test::Bencher;

#[bench]
fn bench_hamming(bencher: &mut Bencher) {
    let a = "ACAAGATGCCATTGTCCCCCGGCCTCCTGCTGCTGCTGCTCTCCGGGG";
    let b = "CCTGGAGGGTGGCCCCACCGGCCGAGACAGCGAGCATATGCAGGAAGC";
    bencher.iter(|| {
        strsim::hamming(a, b).unwrap();
    })
}

#[bench]
fn bench_jaro(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::jaro(a, b);
    })
}

#[bench]
fn bench_jaro_winkler(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::jaro_winkler(a, b);
    })
}

#[bench]
fn bench_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::levenshtein(a, b);
    })
}

fn threshold_inputs() -> (String, String) {
    let a = "abcdefghijklmnop".repeat(256);
    let mut b = a.clone();
    for position in [0, 1024, 2048, 3072] {
        b.replace_range(position..position + 1, "z");
    }
    (a, b)
}

#[bench]
fn bench_levenshtein_then_threshold(bencher: &mut Bencher) {
    let (a, b) = threshold_inputs();
    bencher.iter(|| {
        let distance = strsim::levenshtein(&a, &b);
        if distance <= 8 {
            Some(distance)
        } else {
            None
        }
    })
}

#[bench]
fn bench_bounded_levenshtein(bencher: &mut Bencher) {
    let (a, b) = threshold_inputs();
    bencher.iter(|| strsim::bounded_levenshtein(&a, &b, 8))
}

#[bench]
fn bench_levenshtein_on_u8(bencher: &mut Bencher) {
    bencher.iter(|| {
        strsim::generic_levenshtein(&vec![0u8; 30], &vec![7u8; 31]);
    })
}

#[bench]
fn bench_normalized_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::normalized_levenshtein(a, b);
    })
}

#[bench]
fn bench_osa_distance(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::osa_distance(a, b);
    })
}

#[bench]
fn bench_damerau_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::damerau_levenshtein(a, b);
    })
}

#[bench]
fn bench_normalized_damerau_levenshtein(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::normalized_damerau_levenshtein(a, b);
    })
}

#[bench]
fn bench_sorensen_dice(bencher: &mut Bencher) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";
    bencher.iter(|| {
        strsim::sorensen_dice(a, b);
    })
}
