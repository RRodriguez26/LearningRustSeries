use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hell I'm Raf!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(&message, width, &mut writer).unwrap();
}

// First code wars problems with rust :)
fn no_space(x : String) -> String{
  return x.replace(" ", "");
}

// Second codewars problem with rust
// TODO: FIX THIS METHOD
fn alphabetic(s: &str) -> bool {
    let collection = s.to_lowercase().chars().collect();
    collection.sort_by(|a, b| b.cmp(a));
}