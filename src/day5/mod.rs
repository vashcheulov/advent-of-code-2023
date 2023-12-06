use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;

#[derive(Debug, Clone)]
struct Category {
    name: String,
    ranges: Vec<Range<i64>>,
}

#[derive(Debug, Clone)]
struct Mapper {
    source: Category,
    destination: Category,
}

impl Mapper {
    fn convert(&self, value: i64) -> i64 {
        for (index, range) in self.source.ranges.iter().enumerate() {
            if !range.contains(&value) {
                continue;
            }
            let value_index = value - range.start;
            let destination_range = &self.destination.ranges[index];
            return destination_range.start + value_index;
        }
        return value;
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    mappers: HashMap<String, Mapper>,
}

fn parse_input() -> Almanac {
    let content = fs::read_to_string("src/day5/input.txt").expect("Something went wrong reading the file");
    let chunks = content.split("\n\n").collect::<Vec<&str>>();
    let mut almanac = Almanac { seeds: Vec::new(), mappers: HashMap::new() };

    for (index, chunk) in chunks.iter().enumerate() {
        if index == 0 {
            let input: Vec<&str> = chunk.split(": ").collect();
            let seeds_raw = input[1].split_whitespace().collect::<Vec<&str>>();
            almanac.seeds = seeds_raw.iter().map(|seed| seed.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            continue;
        }

        let mapper_input: &str = chunk.split(" ").collect::<Vec<&str>>()[0];
        let [source, _, destination] = <[&str; 3]>::try_from(mapper_input.split("-").collect::<Vec<&str>>()).ok().unwrap();
        let mut mapper = Mapper {
            source: Category { name: source.to_string(), ranges: Vec::new() },
            destination: Category { name: destination.to_string(), ranges: Vec::new() },
        };
        for (index, map) in chunk.lines().enumerate() {
            if index == 0 { continue; }
            let [destination_start, source_start, range] = <[i64; 3]>::try_from(
                map.split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            ).ok().unwrap();

            mapper.destination.ranges.push(Range { start: destination_start, end: destination_start + range });
            mapper.source.ranges.push(Range { start: source_start, end: source_start + range });
        }
        almanac.mappers.insert(mapper.source.name.to_string(), mapper);
    }
    return almanac;
}

fn find_location(seed: i64, mappers: &HashMap<String, Mapper>) -> i64 {
    let mut mapper_key = String::from("seed");
    let mut value = seed;
    while mappers.contains_key(&mapper_key) {
        let mapper = &mappers[&mapper_key];
        value = mapper.convert(value);
        mapper_key = mapper.destination.name.to_string();
    }
    return value;
}

fn puzzle_1() {
    let almanac = parse_input();
    let bar = ProgressBar::new(almanac.seeds.len() as u64);
    let mut locations: Vec<i64> = Vec::new();

    for seed in almanac.seeds {
        let location = find_location(seed, &almanac.mappers);
        locations.push(location);
        bar.inc(1);
    }
    println!("Min location: {:?}", locations.iter().min())
}

fn puzzle_2() {
    let almanac = parse_input();
    let mut handles = vec![];
    let chunks = almanac.seeds.chunks(2);
    let ranges = chunks.map(|chunk| Range { start: chunk[0], end: chunk[0] + chunk[1] }).collect::<Vec<Range<i64>>>();
    let sum: i64 = ranges.iter().map(|range| range.end - range.start).collect::<Vec<_>>().iter().sum();
    let pb = ProgressBar::new(sum as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} ({percent}%) remaining: ~{eta} {msg}")
        .unwrap());
    let pb = Arc::new(Mutex::new(pb));

    for range in ranges {
        let almanac = almanac.clone();
        let pb = Arc::clone(&pb);
        let handle = thread::spawn(move || {
            let mut min_location: i64 = 0;
            for seed in range.clone() {
                let location = find_location(seed, &almanac.mappers);
                if min_location == 0 || location < min_location {
                    min_location = location;
                }
                if seed % 1000000 == 0 {
                    let bar = pb.lock().unwrap();
                    bar.inc(1000000);
                }
            }
            return min_location;
        });
        handles.push(handle)
    }

    let mut locations: Vec<i64> = Vec::new();
    for i in handles {
        let result = i.join().unwrap();
        locations.push(result);
    }
    println!("Location: {:?}", locations);
    println!("Min location: {:?}", locations.iter().min())
    // Min location: Some(1081323768) - wrong

    // [01:22:48] ███████████████████████████████████████████████████████████████████████████████████████████████████████████████ 1607000000/1606226378 (100%) remaining: ~0s
    // Location: [429431694, 41222968, 186829220, 475876376, 1081323768, 2660261620, 68608231, 2734303452, 154835034, 2353465194]
    // Min location: Some(41222968)
}

pub(crate) fn run() {
    // puzzle_1();
    puzzle_2();
}
