use std::cmp::Ordering;
use std::collections::HashMap;
use std::{fmt, fs};
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Hash, Ord)]
struct Label {
    value: String,
    strength: i32,
}

impl FromStr for Label {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_string();
        return Ok(match s {
            "A" => Label { value, strength: 14 },
            "K" => Label { value, strength: 13 },
            "Q" => Label { value, strength: 12 },
            "J" => Label { value, strength: 11 },
            "T" => Label { value, strength: 10 },
            "9" => Label { value, strength: 9 },
            "8" => Label { value, strength: 8 },
            "7" => Label { value, strength: 7 },
            "6" => Label { value, strength: 6 },
            "5" => Label { value, strength: 5 },
            "4" => Label { value, strength: 4 },
            "3" => Label { value, strength: 3 },
            "2" => Label { value, strength: 2 },
            "1" => Label { value, strength: 1 },
            _ => panic!()
        });
    }
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.value).finish()
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.strength.partial_cmp(&other.strength);
    }
}

impl Label {
    fn strongest() -> Label {
        return Label { value: String::from("A"), strength: 14 };
    }
}

#[derive(PartialEq)]
struct Rule<'a> {
    name: String,
    strength: i32,
    labels: &'a Vec<Label>,
    with_jokers: bool,
}

impl fmt::Debug for Rule<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.name).finish()
    }
}

impl Rule<'_> {
    fn from_labels(labels: &'_ Vec<Label>, with_jokers: bool) -> Rule {
        let mut groups: HashMap<Label, i32> = HashMap::new();
        let mut jokers: i32 = 0;
        for label in labels {
            if with_jokers && label.value == "J" {
                jokers += 1;
                continue;
            }
            *groups.entry(label.clone()).or_insert(0) += 1;
        }

        if groups.is_empty() {
            let binding = Label::strongest();
            groups.insert(binding, 0);
        }
        let strongest_group = groups.iter().last().unwrap();

        let mut strongest_label = strongest_group.0.clone();
        let mut strongest_count = strongest_group.1.clone();
        for (label, count) in &groups {
            if count > &strongest_count {
                strongest_count = *count;
                strongest_label = label.clone();
            }
        }
        groups.insert(strongest_label.clone(), *groups.get(&strongest_label).unwrap() + jokers);

        let count = groups.keys().len();
        let mut values: Vec<&i32> = groups.values().collect::<Vec<&i32>>();
        values.sort();

        match (count, values.as_slice()) {
            (1, [5]) => Rule { name: String::from("Five of a kind"), strength: 7, labels, with_jokers },
            (2, [1, 4]) => Rule { name: String::from("Four of a kind"), strength: 6, labels, with_jokers },
            (2, [2, 3]) => Rule { name: String::from("Full house"), strength: 5, labels, with_jokers },
            (3, [1, 1, 3]) => Rule { name: String::from("Three of a kind"), strength: 4, labels, with_jokers },
            (3, [1, 2, 2]) => Rule { name: String::from("Two pair"), strength: 3, labels, with_jokers },
            (4, [1, 1, 1, 2]) => Rule { name: String::from("One pair"), strength: 2, labels, with_jokers },
            _ => Rule { name: String::from("High card"), strength: 1, labels, with_jokers }
        }
    }
}


impl PartialOrd for Rule<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.strength.partial_cmp(&other.strength);
        if ordering == Some(Ordering::Equal) {
            return self.labels.partial_cmp(&other.labels);
        }
        return ordering;
    }
}


#[derive(Eq, PartialEq, Ord)]
struct Hand {
    labels: Vec<Label>,
    bid: i32,
    with_jokers: bool,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.labels).finish()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rule = Rule::from_labels(&self.labels, self.with_jokers);
        let other_rule = Rule::from_labels(&other.labels, self.with_jokers);
        return rule.partial_cmp(&other_rule);
    }
}

fn parse_input(with_jokers: bool) -> Vec<Hand> {
    let content = fs::read_to_string("src/day7/input.txt").expect("Something went wrong reading the file");
    let mut hands: Vec<Hand> = Vec::new();
    for line in content.lines() {
        let input: Vec<&str> = line.split_whitespace().collect();
        let [labels_input, bid_input] = <[&str; 2]>::try_from(input).ok().unwrap();
        let labels: Vec<Label> = labels_input.chars().map(|l| {
            let mut label = Label::from_str(&l.to_string()).unwrap();
            if with_jokers && label.value == "J" { label.strength = 1 }
            return label;
        }).collect();
        let bid: i32 = bid_input.parse::<i32>().unwrap();
        hands.push(Hand { labels, bid, with_jokers })
    }
    return hands;
}

fn calculate_total_winnings(hands: Vec<Hand>, with_jokers: bool) -> i32 {
    let mut total_winnings = 0;

    for (rank, hand) in hands.iter().enumerate() {
        let rule = Rule::from_labels(&hand.labels, with_jokers);
        total_winnings += hand.bid * (rank as i32 + 1);
        println!("{:?} - {:?}: {:?}", rank, rule, hand)
    }
    return total_winnings;
}

fn puzzle_1() {
    let with_jokers = false;
    let mut hands = parse_input(with_jokers);
    hands.sort();
    let total_winnings = calculate_total_winnings(hands, with_jokers);

    println!("{:?}", total_winnings)
}

fn puzzle_2() {
    let with_jokers = true;
    let mut hands = parse_input(with_jokers);
    hands.sort();
    let total_winnings = calculate_total_winnings(hands, with_jokers);

    println!("{:?}", total_winnings)
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}