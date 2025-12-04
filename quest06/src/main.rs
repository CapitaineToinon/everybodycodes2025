use common::{Part, parse_args};
use std::cmp::min;

fn count(people: &Vec<char>, mentor: char, novice: char) -> usize {
    (0..people.len())
        .filter(|&i| people[i] == mentor)
        .fold(0, |acc, i| {
            (i..people.len()).fold(
                acc,
                |acc, j| if people[j] == novice { acc + 1 } else { acc },
            )
        })
}

fn count_2(
    people: &Vec<char>,
    mentor: char,
    novice: char,
    distance: usize,
    repeat: usize,
) -> usize {
    let len = people.len();
    let total_len = len * repeat;

    let compute_range = |from: usize, to: usize| {
        (from..to)
            .filter(|i| people[*i % len] == mentor)
            .fold(0, |acc, i| {
                let start_j = i.saturating_sub(distance);
                let end_j = min(i + distance, total_len - 1);

                (start_j..=end_j).fold(acc, |acc, j| {
                    if people[j % len] == novice {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
    };

    let mut total = 0;

    // compute the start
    total += compute_range(0, distance);

    // compute the middle
    let middle_len = total_len - (distance + distance);

    // compute first segment
    let segment_total = compute_range(distance, distance + len);

    // count how many times we can fit the people inside
    // the middle
    let segment_count = middle_len / len;

    // add that segment segment_count times
    total += segment_total * segment_count;

    // if middle_len happen to not be divisible by len, compute the remainders
    let remainder = middle_len % len;

    // compute the remainder
    total += compute_range(distance + len, distance + len + remainder);

    // compute the end
    total += compute_range(total_len - distance, total_len);

    total
}

fn main() {
    let args = parse_args();
    let people = args.input.trim().chars().collect::<Vec<_>>();

    let solution = match args.part {
        Part::Part1 => count(&people, 'A', 'a'),
        Part::Part2 => [('A', 'a'), ('B', 'b'), ('C', 'c')]
            .iter()
            .fold(0, |acc, (m, n)| acc + count(&people, *m, *n)),
        Part::Part3 => [('A', 'a'), ('B', 'b'), ('C', 'c')]
            .iter()
            .fold(0, |acc, (m, n)| acc + count_2(&people, *m, *n, 1000, 1000)),
    };

    println!("{}", solution);
}
