use std::{env, io};

pub fn stdin() -> String {
    io::read_to_string(io::stdin()).expect("failed to read input")
}

pub enum Part {
    Part1,
    Part2,
    Part3,
}

pub struct Args {
    pub input: String,
    pub part: Part,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let part = match args.get(1) {
        Some(part) => match part.as_str() {
            "part_1" => Part::Part1,
            "part_2" => Part::Part2,
            "part_3" => Part::Part3,
            _ => panic!("invalid argument"),
        },
        None => panic!("part is required"),
    };

    let input = stdin();

    Args { input, part }
}
