use common::*;
use std::collections::HashSet;

fn parse(input: &String) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|i| i.parse().expect("failed to parse crate"))
        .collect()
}

fn unique(crates: &Vec<i64>) -> Vec<i64> {
    crates
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

fn part_1(crates: Vec<i64>) -> i64 {
    unique(&crates).iter().sum()
}

fn part_2(crates: Vec<i64>) -> i64 {
    let mut sorted = unique(&crates);
    sorted.sort();
    sorted.as_slice()[0..20].iter().sum()
}

fn part_3(crates: Vec<i64>) -> i64 {
    let mut i = 0;
    let mut crates = crates.clone();

    while crates.len() > 0 {
        for c in unique(&crates) {
            crates.remove(crates.iter().position(|&el| el == c).unwrap());
        }

        i += 1;
    }

    i
}

fn main() {
    let args = common::parse_args();
    let crates = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => part_1(crates.clone()),
        Part::Part2 => part_2(crates.clone()),
        Part::Part3 => part_3(crates.clone()),
    };

    println!("{}", solution);
}
