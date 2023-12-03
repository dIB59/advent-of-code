use std::io;

pub trait Counter {
    fn count_char(&self, input: &str) -> i32;
}

pub struct SimpleParenthesesCounter;
pub struct SimplePositionCounter;

impl Counter for SimpleParenthesesCounter {
    fn count_char(&self, input: &str) -> i32 {
        let mut count: i32 = 0;
        for c in input.chars() {
            if c.eq_ignore_ascii_case(&'(') {
                count += 1;
            }
            if c.eq_ignore_ascii_case(&')') {
                count -= 1;
            }
        }
        count
    }
}

impl Counter for SimplePositionCounter {
    fn count_char(&self, input: &str) -> i32 {
        let mut position: i32 = 0;
        let mut count: i32 = 0;
        for c in input.chars() {
            if c.eq_ignore_ascii_case(&'(') {
                count += 1;
            }
            if c.eq_ignore_ascii_case(&')') {
                count -= 1;
            }
            position += 1;
            if count.is_negative() {
                return position;
            }
        }
        -1
    }
}

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

