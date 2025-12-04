use std::collections::{HashMap, HashSet};

use common::{Part, parse_args};

type Name = String;
type Names = Vec<Name>;
type Rules = HashMap<String, HashSet<String>>;

fn parse(input: &String) -> (Names, Rules) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    let names = lines
        .get(0)
        .expect("no names")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<Names>();

    let mut map: Rules = HashMap::new();

    for line in &lines[2..] {
        let (from, to) = line.split_once(" > ").expect("invalid line");
        let from = from.to_string();
        let tos: Vec<String> = to.split(",").map(|s| s.to_string()).collect();
        map.entry(from).or_default().extend(tos);
    }

    (names, map)
}

fn is_valid(name: &Name, rules: &Rules) -> bool {
    for win in name
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(2)
    {
        match win {
            [a, b] if !rules.get(a).expect("invalid rules").contains(b) => {
                return false;
            }
            _ => continue,
        }
    }

    true
}

fn find_valids(names: &Names, rules: &Rules) -> Vec<(usize, Name)> {
    names
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, name)| is_valid(name, rules))
        .collect()
}

fn first_valid(names: &Names, rules: &Rules) -> Option<String> {
    match find_valids(names, rules).first() {
        Some((_, name)) => Some(name.clone()),
        None => None,
    }
}

fn build_name(
    name: &Name,
    rules: &Rules,
    min_len: usize,
    max_length: usize,
    name_cache: &mut HashSet<String>,
    char_cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    // avoid computing a name that was already computed
    // by a different starting name
    if let Some(_) = name_cache.get(name) {
        return 0;
    }

    name_cache.insert(name.clone());

    let (start, end) = name.split_at(name.len() - 1);

    // memoize the function by char and depth
    let char_key = (name.len(), end.to_string());

    if let Some(total) = char_cache.get(&char_key) {
        return *total;
    }

    // first time we're seeing this character as this depth,
    // compute the actual total
    let mut next_total: usize = 0;

    if start.len() < max_length - 1
        && let Some(x) = rules.get(end)
    {
        // only recurse if the name isn't too long
        // and if the last character can be extended
        next_total = x
            .iter()
            .map(|c| {
                let mut next = name.clone();
                next.push_str(c);
                build_name(&next, rules, min_len, max_length, name_cache, char_cache)
            })
            .sum();
    }

    // If name is too small, don't add itself to the total count
    let total = next_total + if name.len() >= min_len { 1 } else { 0 };

    char_cache.insert(char_key, total);

    total
}

fn build_names(names: &Names, rules: &Rules, min_len: usize, max_length: usize) -> usize {
    let mut name_cache: HashSet<String> = HashSet::new();
    let mut char_cache: HashMap<(usize, String), usize> = HashMap::new();

    find_valids(names, rules)
        .iter()
        .map(|(_, name)| {
            build_name(
                name,
                rules,
                min_len,
                max_length,
                &mut name_cache,
                &mut char_cache,
            )
        })
        .sum()
}

fn main() {
    let args = parse_args();
    let (names, rules) = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => first_valid(&names, &rules).expect("failed to find a valid name"),
        Part::Part2 => find_valids(&names, &rules)
            .iter()
            .fold(0, |acc, (i, _)| acc + i + 1)
            .to_string(),
        Part::Part3 => build_names(&names, &rules, 7, 11).to_string(),
    };

    println!("{}", solution);
}
