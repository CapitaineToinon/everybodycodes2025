use common::*;
use std::cmp::{max, min};

struct Quest {
    names: Vec<String>,
    instructions: Vec<String>,
}

fn parse_file(file: &String) -> Quest {
    let lines: Vec<String> = file.split("\n").map(|l| l.to_string()).collect();

    Quest {
        names: lines
            .get(0)
            .unwrap()
            .split(',')
            .map(|l| l.to_string())
            .collect(),
        instructions: lines
            .get(2)
            .unwrap()
            .split(',')
            .map(|l| l.to_string())
            .collect(),
    }
}

fn match_instruction(i: &String) -> i32 {
    let value: i32 = i[1..i.len()].parse().expect("failed to parse instruction");

    match i {
        _ if i.starts_with("L") => -value,
        _ if i.starts_with("R") => value,
        _ => panic!("invalid instruction format"),
    }
}

fn part_1(quest: &Quest) -> String {
    let len: i32 = quest.names.len() as i32;

    let index: usize = quest
        .instructions
        .iter()
        .map(|i| match_instruction(i))
        .fold(0, |a, b| min(max(a + b, 0), len - 1))
        .try_into()
        .expect("failed to convert i32 to usize");

    quest.names[index].clone()
}

fn part_2(quest: &Quest) -> String {
    let len: i32 = quest.names.len() as i32;

    let index: usize = quest
        .instructions
        .iter()
        .map(|i| match_instruction(i))
        .fold(0, |a, b| ((a + b) % len + len) % len)
        .try_into()
        .expect("failed to convert i32 to usize");

    quest.names[index].clone()
}

fn part_3(quest: &Quest) -> String {
    let len: i32 = quest.names.len() as i32;

    let sorted = quest
        .instructions
        .iter()
        .map(|i| match_instruction(i))
        .fold(quest.names.clone(), |mut n, i| {
            let index = (((i % len) + len) % len) as usize;
            n.swap(0, index);
            n
        });

    sorted[0].clone()
}

fn main() {
    let args = parse_args();
    let quest = parse_file(&args.input);

    let solution = match args.part {
        Part::Part1 => part_1(&quest),
        Part::Part2 => part_2(&quest),
        Part::Part3 => part_3(&quest),
    };

    println!("{}", solution);

    ()
}
