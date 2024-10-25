#![feature(test)]

extern crate test;

use funny_crystal_hasher::FunnyHasher;
use rand::{distributions::Alphanumeric, Rng};
use std::hash::{DefaultHasher, Hasher};
use test::Bencher;

#[bench]
fn bench_default_hash(b: &mut Bencher) {
    let string = random_string();

    b.iter(|| {
        let mut hasher = test::black_box(DefaultHasher::new());
        hasher.write(string.as_bytes());
        let _res = hasher.finish();
    })
}

#[bench]
fn bench_funny_hash(b: &mut Bencher) {
    let seeds = (5923811331, 293133411);
    let string = random_string();

    b.iter(|| {
        let mut hasher = test::black_box(FunnyHasher::new(seeds.0, seeds.1));
        hasher.write(string.as_bytes());
        let _res = hasher.finish();
    })
}

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
}
