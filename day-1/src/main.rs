mod counter;

use std::io;
use crate::counter::{Counter, SimpleParenthesesCounter, SimplePositionCounter};


fn main() {
    let mut puzzle_input = String::new();
    io::stdin()
        .read_line(&mut puzzle_input)
        .expect("failed to read from stdin");

    let result = SimpleParenthesesCounter.count_char(&puzzle_input);
    println!("Difference in parenthesis is {} ", result);

    let basement = SimplePositionCounter.count_char(&puzzle_input);
    println!("Santa enters basement at position {}", basement);
}

