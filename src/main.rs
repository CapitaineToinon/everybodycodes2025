use std::{
    cmp::{max, min},
    env,
    error::Error,
    fs,
};

#[derive(Debug, Clone)]
struct Quest {
    names: Vec<String>,
    instructions: Vec<String>,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_file_arg() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(file) => Some(file.to_string()),
        None => None,
    }
}

fn parse_file(file: String) -> Result<Quest> {
    let lines: Vec<String> = fs::read_to_string(file)?
        .split("\n")
        .map(|l| l.to_string())
        .collect();

    let data = Quest {
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
    };

    Ok(data)
}

fn match_instruction(i: String) -> Result<i32> {
    let numbers_str = &i[1..i.len()];
    let value: i32 = numbers_str.parse::<i32>()?;

    let result = match i {
        _ if i.starts_with("L") => -value,
        _ if i.starts_with("R") => value,
        _ => return Err("invalid instruction format".into()),
    };

    Ok(result)
}

fn part_1(quest: Quest) -> Result<String> {
    let len: i32 = quest.names.len().try_into()?;

    let index: usize = quest
        .instructions
        .iter()
        .map(|i| match_instruction(i.into()).unwrap())
        .reduce(|a, b| min(max(a + b, 0), len - 1))
        .unwrap()
        .try_into()
        .expect("failed to convert i32 to usize");

    Ok(quest.names[index].clone())
}

fn part_2(quest: Quest) -> Result<String> {
    let len: i32 = quest.names.len().try_into()?;

    let index: usize = quest
        .instructions
        .iter()
        .map(|i| match_instruction(i.into()).unwrap())
        .reduce(|a, b| ((a + b) % len + len) % len)
        .unwrap()
        .try_into()
        .expect("failed to convert i32 to usize");

    Ok(quest.names[index].clone())
}

fn part_3(quest: Quest) -> Result<String> {
    let len: i32 = quest.names.len().try_into()?;
    let mut names = quest.names.clone();

    quest
        .instructions
        .iter()
        .map(|i| match_instruction(i.into()).unwrap())
        .for_each(|i| {
            let value = ((i % len) + len) % len;
            let index: usize = value.try_into().unwrap();
            names.swap(0, index);
        });

    Ok(names[0].clone())
}

fn main() {
    let file = get_file_arg().expect("file is required");
    let quest = parse_file(file).expect("failed to parse the file");
    let s1 = part_1(quest.clone()).expect("failed to solve part 1");
    let s2 = part_2(quest.clone()).expect("failed to solve part 2");
    let s3 = part_3(quest.clone()).expect("failed to solve part 3");
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    ()
}
