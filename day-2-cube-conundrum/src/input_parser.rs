pub trait Parser {
    fn parse(input: &str) -> Vec<bool>;
}

pub trait SumGameID {
    fn get_sum_of_game_ids(arg: Vec<bool>) -> i32;
}

pub struct CubeGameResult;

impl Parser for CubeGameResult {
    fn parse(input: &str) -> Vec<bool> {

        let mut results: Vec<bool> = Vec::new();
        for line in input.lines() {
            let mut data: Vec<_>= line.split(":").last()
                .expect("Error getting last")
                .split(";")
                .collect();
            let val = Self::is_game_impossible(&mut data);
            results.push(val);
        }
        results
    }
}

impl SumGameID for CubeGameResult {
    fn get_sum_of_game_ids(arg: Vec<bool>) -> i32 {
        let mut sum = 0;
        let mut i = 0;
        arg.iter().for_each(|&value| {
            i += 1;
            if !value { sum += i }
        });
        return sum;
    }
}



impl CubeGameResult {

    fn is_game_impossible(data: &mut Vec<&str>) -> bool {
        for &mut item in data {
            let parts: Vec<&str> = item.split(',').collect();
            for part in parts {
                let tokens: Vec<&str> = part.trim().split_whitespace().collect();
                if tokens.len() == 2 {
                    let color = tokens[1];
                    let number: i32 = tokens[0].parse().expect("Expected a int character");

                    match color {
                        "red" if number > 12 => return true,
                        "green" if number > 13 => return true,
                        "blue" if number > 14 => return true,
                        _ => {}
                    }
                }
            }
        }
        false
    }
}
