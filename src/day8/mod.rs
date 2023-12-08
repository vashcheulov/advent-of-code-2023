use std::collections::HashMap;
use std::fs;
use std::str::FromStr;


#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            _ => panic!()
        });
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, (String, String)>,
}

fn parse_input() -> Network {
    let content = fs::read_to_string("src/day8/input.txt").expect("Something went wrong reading the file");
    let chunks = content.split("\n\n").collect::<Vec<&str>>();
    let [instructions_input, nodes_input] = <[&str; 2]>::try_from(chunks).ok().unwrap();

    let instructions: Vec<Instruction> = instructions_input.chars().map(|c| Instruction::from_str(&c.to_string()).unwrap()).collect::<Vec<Instruction>>();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for node_input in nodes_input.lines() {
        let node = node_input.split(" = ").collect::<Vec<&str>>();
        let [key, directions_input] = <[&str; 2]>::try_from(node).ok().unwrap();
        let directions = directions_input.split(", ").collect::<Vec<&str>>();
        let [left, right] = <[&str; 2]>::try_from(directions).ok().unwrap();
        nodes.insert(key.to_string(), (left[1..].to_string(), right[..right.len() - 1].to_string()));
    }

    return Network { instructions, nodes };
}

fn puzzle_1() {
    let network = parse_input();
    let destination = "ZZZ";
    let mut current_position = "AAA";
    let mut steps = 0;

    while current_position != destination {
        for instruction in &network.instructions {
            let (left, right) = network.nodes.get(current_position).unwrap();
            match instruction {
                Instruction::Left => current_position = left,
                Instruction::Right => current_position = right
            }
            steps += 1;
        }
    }

    println!("{:?}", steps)
}

fn puzzle_2() {
    let network = parse_input();
    let current_nodes: &Vec<String> = &mut network.nodes.iter().map(|node| node.0.clone()).filter(|node| node.ends_with("A")).collect::<Vec<String>>();
    let mut steps: Vec<i64> = Vec::new();

    println!("{:?}", current_nodes);

    for node in current_nodes {
        let mut count = 0;
        let mut current_node = node;
        for instruction in network.instructions.iter().cycle() {
            if current_node.ends_with("Z") {
                break;
            }
            let (left, right) = &network.nodes.get(current_node).unwrap();
            match instruction {
                Instruction::Left => current_node = &left,
                Instruction::Right => current_node = &right
            }
            count += 1;
        }
        steps.push(count);
    }
    let gcd = network.instructions.len() as i64;
    let mut lcm = (steps.pop().unwrap() * steps.pop().unwrap()) / gcd;
    while steps.len() > 0 {
        lcm = (lcm * steps.pop().unwrap()) / gcd;
    }

    println!("{lcm}")
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}
