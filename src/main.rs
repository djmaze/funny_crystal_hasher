use std::{env::args, hash::Hasher, process::exit};

use funny_crystal_hasher::FunnyHasher;

pub fn main() {
    let mut args_iter = args();
    let prog = args_iter.next();

    if args_iter.len() != 3 {
        eprintln!(
            "Usage: {} <SEED 1> <SEED 2> <EMAIL>",
            prog.unwrap_or_default()
        );
        exit(-1);
    };

    let seed1: u64 = args_iter
        .next()
        .unwrap()
        .parse()
        .expect("SEED 1 is no integer");
    let seed2: u64 = args_iter
        .next()
        .unwrap()
        .parse()
        .expect("SEED 2 is no integer");
    let string = args_iter.next().unwrap();

    let mut hasher = FunnyHasher::new(seed1, seed2);
    hasher.write(string.as_bytes());
    println!("{}", hasher.finish());
}
