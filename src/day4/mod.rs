use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    given_numbers: HashSet<i32>,
}

fn parse_input() -> Vec<Card> {
    let content = fs::read_to_string("src/day4/input.txt").expect("Something went wrong reading the file");
    let mut cards: Vec<Card> = Vec::new();
    for line in content.lines() {
        let input: Vec<&str> = line.split(":").collect();
        let [card_input, numbers_input] = <[&str; 2]>::try_from(input).ok().unwrap();

        let numbers_set: Vec<&str> = numbers_input.split("|").collect();
        let [winning_numbers_input, given_numbers_input] = <[&str; 2]>::try_from(numbers_set).ok().unwrap();
        let winning_numbers: Vec<i32> = winning_numbers_input.split_whitespace().flat_map(|s| s.parse::<i32>().ok()).collect();
        let given_numbers: Vec<i32> = given_numbers_input.split_whitespace().flat_map(|s| s.parse::<i32>().ok()).collect();

        let card_input: String = card_input.split_whitespace().collect();
        let card_id: i32 = card_input[4..].parse::<i32>().unwrap();
        cards.push(Card { id: card_id - 1, winning_numbers: HashSet::from_iter(winning_numbers), given_numbers: HashSet::from_iter(given_numbers) })
    }
    return cards;
}

fn puzzle_1() {
    let cards: Vec<Card> = parse_input();
    let mut points = 0;
    for card in cards {
        let numbers = card.winning_numbers.intersection(&card.given_numbers);
        let mut result = 0;
        for _ in numbers {
            match result {
                0 => { result = 1 }
                _ => { result *= 2 }
            }
        }
        points += result;
    }
    println!("{points}")
}


fn find_copies(cards: &Vec<Card>, original_card: &Card) -> i32 {
    let mut stack = vec![original_card.id];
    let mut copies = 0;

    while let Some(current_id) = stack.pop() {
        let current_card = &cards[current_id as usize];
        let numbers = current_card.winning_numbers.intersection(&current_card.given_numbers);

        let mut start = (current_id + 1) as usize;
        let end = start + numbers.count();
        for next_id in start..end {
            stack.push(next_id as i32);
            copies += 1;
        }
    }

    return copies;
}

fn puzzle_2() {
    let cards: Vec<Card> = parse_input();
    let mut total_cards: i32 = 0;

    for card in &cards {
        let copies = find_copies(&cards, &card);
        total_cards += copies + 1;
    }
    println!("{total_cards}");
    // 19499881
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}