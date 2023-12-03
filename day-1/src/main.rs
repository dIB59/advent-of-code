use crate::input::PUZZLE_INPUT;
use crate::reader::{FirstAndLastInteger, Reader};

mod input;
mod reader;

fn main() {

    let numbers = FirstAndLastInteger::read(PUZZLE_INPUT);
    let result: i32 = numbers.iter().sum();
    print!("{}", result)

}

