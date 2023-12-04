pub trait Reader {
    fn read(input: &str) -> Vec<i32>;
}

pub struct FirstAndLastInteger;
pub struct FirstAndLastIntegerWords;

impl Reader for FirstAndLastInteger {
    fn read(input: &str) -> Vec<i32> {
        let mut numbers = Vec::new();
        for line in input.lines() {
            let chars: Vec<char> = line.chars()
                .filter(|c| c.is_numeric())
                .collect();
            Self::first_last_ints(&mut numbers, chars);
        }
        return numbers;
    }
}

impl FirstAndLastInteger {
    fn first_last_ints(numbers: &mut Vec<i32>, chars: Vec<char>) {
        let mut some: String = String::from("");


        if let Some(c) = chars.first() {
            some.push(*c);
        }
        if let Some(c) = chars.last() {
            some.push(*c);
        }
        numbers.push(some.parse::<i32>()
            .expect("Failed to parse as i32"));
    }
}