extern crate rand;

use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use rand::{thread_rng, Rng};

fn main() {
    let f = File::open("eff_large_wordlist.txt").expect("wordlist doesn't exist");

    let mut words: Vec<String> = Vec::with_capacity(7776);
    for line in BufReader::new(f).lines() {
        words.push(line.unwrap());
    }

    let rounds: u8 = env::args().nth(1).expect("must pass number of rounds as argument")
                                       .parse().expect("could not convert argument to u8");
    let mut rng = thread_rng();
    for _ in 0..rounds {
        let x: usize = rng.gen_range(0, 7775);
        println!("{}", words[x]);
    }
}
