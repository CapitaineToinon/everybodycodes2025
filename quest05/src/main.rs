mod sword;

use crate::sword::Sword;
use common::{Part, parse_args};

fn parse_sword(input: &String) -> Sword {
    let (a, b) = input.trim().split_once(":").expect("invalid input");

    let (id, steps) = (
        a.parse().expect("failed to parse id"),
        b.split(",")
            .map(|n| n.trim().parse().expect("failed to parse step"))
            .collect(),
    );

    Sword::from_input(id, steps)
}

fn parse_swords(input: &String) -> Vec<Sword> {
    input
        .trim()
        .split("\n")
        .map(|line| parse_sword(&line.to_string()))
        .collect()
}

fn main() {
    let args = parse_args();

    let solution = match args.part {
        Part::Part1 => parse_sword(&args.input).quality(),
        Part::Part2 => {
            let mut swords = parse_swords(&args.input);

            swords.sort_by(|a, b| a.cmp_simple(&b));

            let (worst, best) = match (swords.first(), swords.last()) {
                (None, _) | (_, None) => panic!("failed to find min and max"),
                (Some(a), Some(b)) => (a, b),
            };

            best.quality() - worst.quality()
        }
        Part::Part3 => {
            let mut swords = parse_swords(&args.input);

            swords.sort_by(|a, b| a.cmp_full(&b));

            swords
                .iter()
                .map(|sword| sword.id)
                .rev()
                .enumerate()
                .fold(0, |acc, (i, id)| acc + id * ((i + 1) as i64))
        }
    };

    println!("{}", solution);
}
