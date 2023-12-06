use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Part {
    SYMBOL(String),
    NUMBER(i32),
    PERIOD,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        if c.is_digit(10) {
            return Ok(Part::NUMBER(c.to_digit(10).unwrap() as i32));
        } else if c == '.' {
            return Ok(Part::PERIOD);
        } else if c.is_ascii_punctuation() {
            return Ok(Part::SYMBOL(s[0..1].to_string()));
        }
        return Err(());
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    position: (usize, Range<usize>),
}

impl Number {
    fn add_part(&mut self, number: i32) {
        let mut value_str = self.value.to_string();
        value_str.push_str(&*number.to_string());
        self.value = value_str.parse::<i32>().unwrap();
    }

    fn has_position(&self, pos: (usize, usize)) -> bool {
        return self.position.0 == pos.0 && (self.position.1.end == pos.1 || self.position.1.contains(&pos.1));
    }

    fn surroundings(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        let row = self.position.0;
        let start: usize;
        if self.position.1.start == 0 {
            start = self.position.1.start;
        } else {
            start = self.position.1.start - 1;
        }
        let end = self.position.1.end + 1;

        for column in start..end + 1 {
            if row != 0 {
                positions.push((row - 1, column));
            }
            positions.push((row, column));
            positions.push((row + 1, column))
        }

        return positions;
    }
}

#[derive(Debug)]
struct EngineSchematic {
    rows: Vec<Vec<Part>>,
    numbers: Vec<Number>,
}

impl EngineSchematic {
    fn fits(&self, position: (&usize, &usize)) -> bool {
        let height = self.rows.len();
        let width = self.rows[0].len();
        return &0 <= position.0 && position.0 < &height && &0 <= position.1 && position.1 < &width;
    }
}

fn parse_input() -> EngineSchematic {
    let content = fs::read_to_string("src/day3/input.txt").expect("Something went wrong reading the file");
    let mut rows: Vec<Vec<Part>> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    for (line_index, line) in content.lines().into_iter().enumerate() {
        let mut row: Vec<Part> = Vec::new();
        let mut current_number: Option<Number> = None;
        for (index, char) in line.chars().enumerate() {
            let part = Part::from_str(&char.to_string()).unwrap();
            match part {
                Part::NUMBER(num) => {
                    if let Some(ref mut number) = current_number {
                        number.add_part(num);
                        number.position.1.end += 1;
                    } else {
                        current_number = Some(Number { value: num, position: (line_index, Range { start: index, end: index }) });
                    }
                }
                _ => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                        current_number = None;
                    }
                }
            }
            row.push(part);
        }
        if let Some(number) = current_number.take() {
            numbers.push(number);
        }
        rows.push(row)
    }
    return EngineSchematic { rows, numbers };
}

fn puzzle_1() {
    let engine_schematic = parse_input();
    let mut sum = 0;
    for number in &engine_schematic.numbers {
        for (row, column) in number.surroundings() {
            if engine_schematic.fits((&row, &column)) {
                let part = engine_schematic.rows[row][column].clone();
                match part {
                    Part::SYMBOL(_) => {
                        sum += number.value;
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{:?}", sum)
}

fn puzzle_2() {
    let engine_schematic = parse_input();
    let mut ratio = 0;
    for (row_index, row) in engine_schematic.rows.iter().enumerate() {
        for (col_index, part) in row.iter().enumerate() {
            match part {
                Part::SYMBOL(sym) => {
                    if sym != "*" { continue; }
                    let mut matched: Vec<Number> = Vec::new();
                    let surroundings = vec![
                        (row_index - 1, col_index - 1),
                        (row_index - 1, col_index),
                        (row_index - 1, col_index + 1),
                        (row_index, col_index - 1),
                        (row_index, col_index + 1),
                        (row_index + 1, col_index - 1),
                        (row_index + 1, col_index),
                        (row_index + 1, col_index + 1),
                    ];
                    for number in &engine_schematic.numbers {
                        for sur in &surroundings {
                            if number.has_position(*sur) {
                                matched.push(number.clone());
                                break;
                            }
                        }
                    }
                    if matched.len() == 2 {
                        ratio += matched[0].value * matched[1].value;
                    }
                }
                _ => {}
            }
        }
    }
    println!("{ratio}")
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}