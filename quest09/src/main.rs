use common::{Part, parse_args};
use std::collections::{HashMap, HashSet};

type Dna = (usize, Vec<u128>);
type Person = (usize, Dna);

fn map_char(c: u8) -> u128 {
    match c {
        c if c == 'A' as u8 => 0b0001,
        c if c == 'C' as u8 => 0b0010,
        c if c == 'T' as u8 => 0b0100,
        c if c == 'G' as u8 => 0b1000,
        _ => panic!("invalid dna sequence"),
    }
}

fn get_similarity((len1, dna1): &Dna, (len2, dna2): &Dna, (len3, dna3): &Dna) -> Option<usize> {
    // bail if the dna aren't all the same size
    if len1 != len2 || len1 != len3 {
        return None;
    }

    let mut score_a = 0;
    let mut score_b = 0;

    for (c, (p1, p2)) in dna1.iter().zip(dna2.iter().zip(dna3)) {
        // bail if not related
        if (c ^ (c & p1)) & (c ^ (c & p2)) != 0 {
            return None;
        }

        // start computing the similary score
        for i in 0..32 {
            let s = i * 4;
            let m = 0b1111;
            let c = (c >> s) & m;
            let a = (p1 >> s) & m;
            let b = (p2 >> s) & m;

            if c == 0 || a == 0 || b == 0 {
                break;
            }

            if a == c {
                score_a += 1;
            }

            if b == c {
                score_b += 1;
            }
        }
    }

    Some(score_a * score_b)
}

fn parse_dna(dna: &String) -> Dna {
    let len = dna.len();

    let dna = dna
        .bytes()
        .collect::<Vec<u8>>()
        .chunks(32) // sizeof u128
        .map(|bytes| {
            bytes
                .iter()
                .enumerate()
                .fold(0u128, |acc, (i, &byte)| acc + (map_char(byte) << (4 * i)))
        })
        .collect::<Vec<u128>>();

    (len, dna)
}

fn parse(input: &String) -> Vec<Person> {
    input
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .map(|line| match line.split_once(":") {
            Some((id, dna)) => (
                id.parse().expect("failed to parse id"),
                parse_dna(&dna.to_string()),
            ),
            _ => panic!("invalid input"),
        })
        .collect::<Vec<Person>>()
}

fn find_parents((id, dna): &Person, everyone: &Vec<Person>) -> Option<(usize, usize, usize)> {
    for a in 0..everyone.len() {
        for b in a..everyone.len() {
            let (id_a, dna_a) = &everyone[a];
            let (id_b, dna_b) = &everyone[b];

            if id == id_a || id_b == id {
                continue;
            }

            if let Some(score) = get_similarity(&dna, &dna_a, &dna_b) {
                return Some((*id_a, *id_b, score));
            }
        }
    }

    None
}

fn find_first_relation(people: &Vec<Person>) -> Option<(usize, usize, usize, usize)> {
    for child in people {
        if let Some((id_a, id_b, score)) = find_parents(&child, &people) {
            let &(id, _) = child;
            return Some((id, id_a, id_b, score));
        }
    }

    None
}

fn compute_similarities(everyone: &Vec<Person>) -> usize {
    let mut total = 0;

    for child in everyone {
        if let Some((_, _, score)) = find_parents(child, &everyone) {
            total += score;
        }
    }

    total
}

fn build_graph(everyone: &Vec<Person>) -> HashMap<usize, HashSet<usize>> {
    let mut relations: HashMap<usize, HashSet<usize>> = HashMap::new();

    for person in everyone {
        if let Some((id_a, id_b, _)) = find_parents(person, &everyone) {
            let (id, _) = person;
            relations.entry(*id).or_default().extend([id_a, id_b]);
            relations.entry(id_a).or_default().extend([id]);
            relations.entry(id_b).or_default().extend([id]);
        }
    }

    relations
}

fn family_size(
    member: usize,
    relations: &HashMap<usize, HashSet<usize>>,
    family: &mut HashSet<usize>,
) {
    if let Some(_) = family.get(&member) {
        return;
    }

    family.insert(member);

    if let Some(related) = relations.get(&member) {
        related
            .iter()
            .for_each(|member| family_size(*member, &relations, family));
    }
}

fn find_biggest_family(relations: &HashMap<usize, HashSet<usize>>) -> usize {
    relations
        .keys()
        .map(|child| {
            let mut family: HashSet<usize> = HashSet::new();
            family_size(*child, &relations, &mut family);
            family
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("graph is empty")
        .iter()
        .sum()
}

fn main() {
    let args = parse_args();
    let people = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => {
            let (_, _, _, score) =
                find_first_relation(&people).expect("failed to find a valid child");
            score
        }
        Part::Part2 => compute_similarities(&people),
        Part::Part3 => {
            let graph = build_graph(&people);
            find_biggest_family(&graph)
        }
    };

    dbg!(solution);
}
