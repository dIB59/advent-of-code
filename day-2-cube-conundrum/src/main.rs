use crate::input::PUZZLE_INPUT;
use crate::input_parser::{CubeGameResult, Parser, SumGameID};

mod input;
mod input_parser;

fn main() {
    println!("Hello, world!");
    let some = CubeGameResult::parse(PUZZLE_INPUT);
    let res = CubeGameResult::get_sum_of_game_ids(some);
    println!("{}", res)
}
