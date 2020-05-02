# diceware-gen
![Crates.io](https://img.shields.io/crates/v/diceware-gen)

This program generates random passphrases based on
[EFF's long wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).

# usage
```rust
extern crate diceware_gen;
use diceware_gen::DicewareGen;

fn main() {
    let dice = DicewareGen::new().unwrap();
    for word in dice.gen(10) {
        println!("{}", word);
    }
}
```
