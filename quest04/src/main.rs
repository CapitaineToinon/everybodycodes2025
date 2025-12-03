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

fn part_1(gears: &Gears) -> i64 {
    gears
        .windows(2)
        .fold(2025.0, |acc, w| match w {
            [(a, _), (b, _)] => acc * (a / b),
            _ => panic!("invalid window"),
        })
        .floor() as i64
}

fn part_2(gears: &Gears) -> i64 {
    // same as part 1 but need to ceil, traverse the windows
    // in reverse direction and divide instead of mutliply the ratios
    gears
        .windows(2)
        .rev()
        .fold(10000000000000.0, |acc, w| match w {
            [(a, _), (b, _)] => acc / (a / b),
            _ => panic!("invalid window"),
        })
        .ceil() as i64
}

fn part_3(gears: &Gears) -> i64 {
    gears
        .windows(2)
        .fold(100.0, |acc, w| match w {
            [(a1, a2), (b, _)] => {
                acc * match (a1, a2) {
                    (x, None) => x / b,
                    (_, Some(x)) => x / b,
                }
            }
            _ => panic!("invalid window"),
        })
        .floor() as i64
}

fn main() {
    let args = parse_args();
    let gears = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => part_1(&gears),
        Part::Part2 => part_2(&gears),
        Part::Part3 => part_3(&gears),
    };

    println!("{}", solution);
}
