use std::fs;
use std::iter::Iterator;

fn parse_input() -> String {
    let content = fs::read_to_string("src/day1/input.txt").expect("Something went wrong reading the file");
    return content;
}

const SPELLED_NUMBERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_spelled(spelled_number: String) -> Option<u32> {
    if SPELLED_NUMBERS.iter().any(|(spelled, _)| spelled.starts_with(&spelled_number)) {
        let spelled_number = SPELLED_NUMBERS.iter().find(|(spelled, _)| spelled == &spelled_number);
        if spelled_number.is_some() {
            return Option::Some(spelled_number.unwrap().1);
        }
    }
    return Option::None;
}

fn find_digits(line: String, with_spelled: bool) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    for (index, char) in line.chars().enumerate() {
        if with_spelled && char.is_alphabetic() {
            let mut spelled_number = String::new();
            let mut found = false;
            spelled_number.push(char);
            for next_char in line.chars().skip(index + 1) {
                let spelled = get_spelled(spelled_number.clone());
                if spelled.is_some() {
                    digits.push(spelled.unwrap());
                    found = true;
                    break;
                }
                spelled_number.push(next_char);
            }
            let spelled = get_spelled(spelled_number.clone());
            if !found && spelled.is_some() {
                digits.push(spelled.unwrap());
                break;
            }
            continue;
        }
        if char.is_numeric() {
            digits.push(char.to_digit(10).unwrap());
        }
    }
    return digits;
}

fn calculate(content: String, with_spelled: bool) -> i32 {
    let mut sum: i32 = 0;

    for (index, line) in content.lines().enumerate() {
        let digits = find_digits(line.to_string(), with_spelled);
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i32>().expect("Invalid input format");
        println!("{}:{:?},{}", index, digits, number);
        sum += number
    }

    return sum;
}


fn puzzle_1() {
    let content = parse_input();
    let sum: i32 = calculate(content, false);

    println!("Sum is {sum}")
}

fn puzzle_2() {
    let content = parse_input();
    let sum: i32 = calculate(content, true);

    println!("Sum is {sum}")
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}