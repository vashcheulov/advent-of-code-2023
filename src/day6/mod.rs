use std::fs;
use std::iter::zip;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug)]
struct Race {
    time: Duration,
    distance: i64,
}

#[derive(Debug)]
struct ToyBoat {
    speed: u128,
}

impl ToyBoat {
    fn charge(duration: Duration) -> ToyBoat {
        return ToyBoat { speed: duration.as_millis() };
    }

    fn release(&self, time_left: Duration) -> i64 {
        return (self.speed * time_left.as_millis()) as i64;
    }
}

fn parse_input() -> Vec<Race> {
    let content = fs::read_to_string("src/day6/input.txt").expect("Something went wrong reading the file");
    let lines = content.lines().collect::<Vec<&str>>();

    let input: Vec<&str> = lines[0].split(":").collect();
    let [_, time_input] = <[&str; 2]>::try_from(input).ok().unwrap();
    let times = time_input.split_whitespace().collect::<Vec<&str>>().iter().map(|time| Duration::from_millis(time.parse::<u64>().unwrap())).collect::<Vec<Duration>>();

    let input: Vec<&str> = lines[1].split(":").collect();
    let [_, distance_input] = <[&str; 2]>::try_from(input).ok().unwrap();
    let distances = distance_input.split_whitespace().collect::<Vec<&str>>().iter().map(|distance| distance.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut races: Vec<Race> = Vec::new();
    for (time, distance) in zip(times, distances) {
        races.push(Race { time, distance });
    }
    return races;
}

fn parse_input_for_single_race() -> Race {
    let content = fs::read_to_string("src/day6/input.txt").expect("Something went wrong reading the file");
    let lines = content.lines().collect::<Vec<&str>>();

    let input: Vec<&str> = lines[0].split(":").collect();
    let [_, time_input] = <[&str; 2]>::try_from(input).ok().unwrap();
    let time = Duration::from_millis(time_input.split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap() as u64);

    let input: Vec<&str> = lines[1].split(":").collect();
    let [_, distance_input] = <[&str; 2]>::try_from(input).ok().unwrap();
    let distance = distance_input.split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();

    return Race { time, distance };
}


fn puzzle_1() {
    let races = parse_input();
    let mut result: i32 = 1;

    for race in races {
        let mut beatable_ways: i32 = 0;

        for millis in 1..race.time.as_millis() {
            let boat = ToyBoat::charge(Duration::from_millis(millis as u64));
            let time_left = Duration::from_millis((race.time.as_millis() - millis) as u64);
            if boat.release(time_left) > race.distance {
                beatable_ways += 1;
            }
        }

        result *= beatable_ways;
    }

    println!("{:?}", result)
}

fn puzzle_2() {
    let race = parse_input_for_single_race();

    let pb = ProgressBar::new(race.time.as_millis() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} ({percent}%) remaining: ~{eta} {msg}")
        .unwrap());

    let mut beatable_ways: i32 = 0;

    for millis in 1..race.time.as_millis() {
        let boat = ToyBoat::charge(Duration::from_millis(millis as u64));
        let time_left = Duration::from_millis((race.time.as_millis() - millis) as u64);
        if boat.release(time_left) > race.distance {
            beatable_ways += 1;
        }
        pb.inc(1)
    }


    println!("{:?}", beatable_ways)
}


pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}
