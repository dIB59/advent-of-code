use crate::input::PUZZLE_INPUT;
use crate::input_parser::{Parser, Points, Scratchcards};

mod input;
mod input_parser;

fn main() {
    println!("Hello, world!");
    let scratchcards_numbers = Scratchcards::parse(PUZZLE_INPUT);
    let scratchcards_points = Scratchcards::calculate_points(scratchcards_numbers);
    print!("{}", scratchcards_points);
    return;
}
