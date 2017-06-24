extern crate rand;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum DiceError {
    Io(io::Error),
}

impl From<io::Error> for DiceError {
    fn from(err: io::Error) -> DiceError {
        DiceError::Io(err)
    }
}

pub struct DicewareGen {
    words: Vec<String>,
    rng: rand::ThreadRng,
}

impl DicewareGen {
    pub fn new() -> Result<DicewareGen, DiceError> {
        Ok(DicewareGen {
            words: {
                // The executable's parent should always exist, since every file needs a parent.
                let f = File::open(env::current_exe()?.parent().unwrap().join("eff_large_wordlist.txt"))?;
                BufReader::new(f).lines().map(|line| {
                    line
                }).collect::<Result<Vec<String>, io::Error>>()?
            },
            rng: thread_rng(),
        })
    }

    pub fn gen(mut self, rounds: u8) -> Vec<String> {
        (0..rounds).map(|_| {
            let x: usize = self.rng.gen_range(0, 7775);
            self.words[x].clone()
        }).collect::<Vec<String>>()
    }
}
