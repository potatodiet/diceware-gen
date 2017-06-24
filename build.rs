fn main() {
    if std::env::var("PROFILE").unwrap() == "debug" {
        std::fs::copy("eff_large_wordlist.txt", "target/debug/eff_large_wordlist.txt").unwrap();
    }
}
