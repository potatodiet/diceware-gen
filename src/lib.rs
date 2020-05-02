use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::prelude::*;

#[derive(Debug)]
pub enum DiceError {
    Io(io::Error),
    Var(env::VarError),
}

impl From<io::Error> for DiceError {
    fn from(err: io::Error) -> DiceError {
        DiceError::Io(err)
    }
}

impl From<env::VarError> for DiceError {
    fn from(err: env::VarError) -> DiceError {
        DiceError::Var(err)
    }
}

pub struct DicewareGen {
    words: Vec<String>,
    rng: ThreadRng,
}

impl DicewareGen {
    pub fn new() -> Result<DicewareGen, DiceError> {
        let dirs = vec!(
            // if env var doesn't exist, then try reading from current directory.
            match env::var("DICEWARE_GEN_WORDLISTS") {
                Ok(p) => p,
                Err(_) => String::from(""),
            },
            String::from("/usr/share/diceware-gen/"),
        );
        let f = read_order(dirs, "eff_large_wordlist.txt")?;

        Ok(DicewareGen {
            words: {
                BufReader::new(f)
                    .lines()
                    .map(|l| l.expect("Error parsing wordlist"))
                    .collect()
            },
            rng: thread_rng(),
        })
    }

    pub fn gen(mut self, rounds: u8) -> Vec<String> {
        (0..rounds).map(|_| {
            let x: usize = self.rng.gen_range(0, 7775);
            self.words[x].clone()
        }).collect()
    }
}

fn read_order(dirs: Vec<String>, f: &str) -> Result<File, DiceError> {
    for d in dirs {
        let p = Path::new(&d).join(f);

        match File::open(p) {
            Ok(f) => return Ok(f),
            Err(_) => {},
        }
    }

    Err(DiceError::Io(io::Error::new(io::ErrorKind::Other, "could not read wordlist")))
}
