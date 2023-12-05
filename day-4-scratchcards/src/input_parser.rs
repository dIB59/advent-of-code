pub trait Parser {
    fn parse(input: &str) -> Vec<Vec<Vec<i32>>>;
}

pub trait Points {
    fn calculate_points(input: Vec<Vec<Vec<i32>>>) -> i32;
}

pub struct Scratchcards;

impl Points for Scratchcards {
    fn calculate_points(input: Vec<Vec<Vec<i32>>>) -> i32 {
        let mut points = 0;
        for i in 0..input.len() {
            let power = number_of_duplicates(&input[i][0], &input[i][1]);
            println!("{}", i);
            println!("{}", power);
            if power == 0 { continue; }
            if power == 1 {
                points += 1;
                continue;
            }
            println!("points : {}", points);
            points += 2i32.pow(power as u32 - 1)
        }
        return points;
    }
}

impl Parser for Scratchcards {
    fn parse(input: &str) -> Vec<Vec<Vec<i32>>> {
        let mut final_result = Vec::new();
        let temp: Vec<_> = input
            .lines()
            .map(|line| final_result.push(get_numbers_and_winning_numbers(line))).collect();
        return final_result;
    }
}

fn get_numbers_and_winning_numbers(temp: &str) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let temp1 = temp
        .split(":").last().expect("Please make sure your input has a \":\"");
    let parts: Vec<&str> = temp1.split("|").map(|s| s.trim()).collect();
    let winning_numbers: Vec<i32> = extract_numbers(parts[0]);
    let your_numbers: Vec<i32> = extract_numbers(parts[1]);
    res.push(your_numbers);
    res.push(winning_numbers);
    return res;
}

fn extract_numbers(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn number_of_duplicates(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut count = 0;

    for &num in list1 {
        if list2.contains(&num) {
            count += 1;
        }
    }
    count
}

