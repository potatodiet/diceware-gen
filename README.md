# diceware-gen
![Crates.io](https://img.shields.io/crates/v/diceware-gen)

This program generates random passphrases based on
[EFF's long wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).

# Requirements
* Rust v1.31+

# Building
    cargo build --release

The compiled binary will be located at target/release/diceware-gen

# Usage
    diceware-gen N

Where N = the number of random words to produce.

# Licensing
Licensed under GPL-3.0-only
