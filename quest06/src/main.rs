use std::cmp::{max, min};

use common::{Part, parse_args};

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn count(people: &[u8], mentor: char, novice: char) -> i64 {
    let mut total = 0;

    for (i, person) in people.iter().enumerate() {
        if *person == (mentor as u8) {
            for next in &people[i..people.len()] {
                if *next == novice as u8 {
                    total += 1;
                }
            }
        }
    }

    total
}

fn count_2(people: &[u8], mentor: char, novice: char, distance: i64, repeat: i64) -> i64 {
    let len = people.len() as i64;
    let total_len = len * repeat;

    let compute_range = |from: i64, to: i64| {
        let mut total = 0;
        let mentor_byte = mentor as u8;
        let novice_byte = novice as u8;

        for i in from..to {
            let p = people[modulo(i, len) as usize];

            if p != novice_byte {
                continue;
            }

            let start_j = max(i - distance, 0);
            let end_j = min(i + distance, total_len - 1);

            for j in start_j..=end_j {
                if people[modulo(j, len) as usize] == mentor_byte {
                    total += 1;
                }
            }
        }

        total
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
    let people = args.input.trim().as_bytes();

    let solution = match args.part {
        Part::Part1 => count(people, 'A', 'a'),
        Part::Part2 => [('A', 'a'), ('B', 'b'), ('C', 'c')]
            .iter()
            .fold(0, |acc, (m, n)| acc + count(people, *m, *n)),
        Part::Part3 => [('A', 'a'), ('B', 'b'), ('C', 'c')]
            .iter()
            .fold(0, |acc, (m, n)| acc + count_2(people, *m, *n, 1000, 1000)),
    };

    println!("{}", solution);
}
