use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let message;

    // run "cargo run --features "clippy"" to produce clippy art and message
    if cfg!(feature = "clippy") {
        message = "Hello, I'm Clippy!";
    } else {
        message = "Hello, I'm Raf!";
    }
    
    speak(message);
}

fn speak(message: &str) {
    let stdout = stdout();
    let message = String::from(message);
    let width_intro = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(&message, width_intro, &mut writer).unwrap();
}

// First code wars problems with rust :)
// fn no_space(x : String) -> String{
//   return x.replace(" ", "");
// }

// Second codewars problem with rust
// TODO: FIX THIS METHOD
// fn alphabetic(s: &str) -> bool {
//     let collection = s.to_lowercase().chars().collect();
//     collection.sort_by(|a, b| b.cmp(a));
// }