use common::{Part, parse_args};

type Gears = Vec<(f64, Option<f64>)>;

fn parse(input: &String) -> Gears {
    input
        .trim()
        .split("\n")
        .map(|s| match s.split_once("|") {
            Some((a, b)) => (
                a.parse::<f64>().expect("failed to parse first gear"),
                Some(b.parse::<f64>().expect("failed to parse second gear")),
            ),
            None => (s.parse::<f64>().expect("failed to parse gear"), None),
        })
        .collect()
}

fn left_to_right(gears: &Gears, rotations: f64) -> i64 {
    gears
        .windows(2)
        .fold(rotations, |acc, w| match w {
            [(a1, a2), (b, _)] => {
                acc * match (a1, a2) {
                    (_, Some(x)) => x / b,
                    (x, None) => x / b,
                }
            }
            _ => panic!("invalid window"),
        })
        .floor() as i64
}

fn right_to_left(gears: &Gears, rotations: f64) -> i64 {
    // same as left_to_right but need to ceil, traverse the windows
    // in reverse direction and divide instead of mutliply the ratios
    gears
        .windows(2)
        .rev()
        .fold(rotations, |acc, w| match w {
            // technically we should check for mounted gears
            // also when going from right to left but it's
            // not required for part_2
            [(a, _), (b, _)] => acc / (a / b),
            _ => panic!("invalid window"),
        })
        .ceil() as i64
}

fn main() {
    let args = parse_args();
    let gears = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => left_to_right(&gears, 2025.0),
        Part::Part2 => right_to_left(&gears, 10000000000000.0),
        Part::Part3 => left_to_right(&gears, 100.0),
    };

    println!("{}", solution);
}
