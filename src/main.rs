use diceware_gen::DicewareGen;
use std::env;

fn main() {
    let dice = DicewareGen::new().unwrap();

    let rounds: u8 = env::args().nth(1).expect("must pass number of rounds as argument")
                                .parse().expect("could not convert argument to u8");
    for word in dice.gen(rounds) {
        println!("{}", word);
    }
}
