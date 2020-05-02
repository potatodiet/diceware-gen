use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use rand::prelude::*;

pub struct DicewareGen {
    words: Vec<String>,
    rng: ThreadRng,
}

impl DicewareGen {
    pub fn new() -> Result<DicewareGen, Box<dyn Error>> {
        // if env var doesn't exist, then try reading from current directory.
        let dir = match env::var("DICEWARE_GEN_WORDLISTS") {
            Ok(p) => p,
            Err(_) => String::from("/usr/share/diceware-gen/"),
        };
        let p = Path::new(&dir).join("eff_large_wordlist.txt");
        let f = File::open(p)?;

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
            let x = self.rng.gen_range(0, 7775);
            self.words[x].clone()
        }).collect()
    }
}
