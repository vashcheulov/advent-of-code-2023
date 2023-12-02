use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Cube {
    color: Color,
}


#[derive(Debug)]
struct Sample {
    cubes: Vec<Cube>,
}

impl Sample {
    fn can_fit(&self, count: &i32, color: &Color) -> bool {
        let mut remaining = count.clone();
        for cube in &self.cubes {
            if cube.color == *color {
                remaining -= 1
            }
        }
        return remaining >= 0;
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    samples: Vec<Sample>,

}

impl Game {
    fn max_cubes<'a>(&self, color: Color) -> Option<usize> {
        return self.samples.iter().map(|sample| sample.cubes.iter().filter(|cube| cube.color == color).count()).max();
    }
}

struct Bag {
    sets: Vec<(i32, Color)>,
}

fn parse_input() -> Vec<Game> {
    let content = fs::read_to_string("src/day2/input.txt").expect("Something went wrong reading the file");
    let mut games: Vec<Game> = Vec::new();

    for line in content.lines() {
        let mut samples: Vec<Sample> = Vec::new();

        let input: Vec<&str> = line.split(":").collect();
        let [game_input, samples_input] = <[&str; 2]>::try_from(input).ok().unwrap();
        let samples_set: Vec<&str> = samples_input.split(";").collect();

        for sample in samples_set {
            let mut cubes: Vec<Cube> = Vec::new();
            let cubes_set: Vec<&str> = sample.split(",").collect();

            for cube_set in cubes_set {
                let cube_set: Vec<&str> = cube_set[1..].split(" ").collect();
                let [count, color_str] = <[&str; 2]>::try_from(cube_set).ok().unwrap();
                let color = Color::from_str(color_str).unwrap();

                for _ in 0..count.parse::<i32>().unwrap() {
                    cubes.push(Cube { color })
                }
            }

            samples.push(Sample { cubes })
        }

        let game_input: Vec<&str> = game_input.split(" ").collect();
        let [_, game_id] = <[&str; 2]>::try_from(game_input).ok().unwrap();
        let game_id = game_id.parse::<i32>().unwrap();
        games.push(Game { id: game_id, samples })
    }
    return games;
}


fn puzzle_1() {
    let games = parse_input();
    let bag = Bag { sets: vec![(12, Color::RED), (13, Color::GREEN), (14, Color::BLUE)] };
    let mut sum: i32 = 0;

    for game in games {
        let is_possible = game.samples.iter().all(|sample| bag.sets.iter().all(|(count, color)| sample.can_fit(count, color)));
        if is_possible { sum += game.id }
    }

    println!("{:?}", sum)
}

fn puzzle_2() {
    let games = parse_input();
    let mut sum: i32 = 0;

    for game in games {
        let bag = Bag {
            sets: vec![
                (game.max_cubes(Color::RED).unwrap_or(0) as i32, Color::RED),
                (game.max_cubes(Color::GREEN).unwrap_or(0) as i32, Color::GREEN),
                (game.max_cubes(Color::BLUE).unwrap_or(0) as i32, Color::BLUE),
            ]
        };
        let power: i32 = bag.sets.iter().map(|(count, _)| count).product();
        sum += power
    }

    println!("{:?}", sum)
}

pub(crate) fn run() {
    // puzzle_1()
    puzzle_2()
}