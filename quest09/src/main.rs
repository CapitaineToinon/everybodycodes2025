use std::collections::{HashMap, HashSet};

use common::{Part, parse_args};

type Person = (usize, String);

fn parse(input: &String) -> Vec<Person> {
    input
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .map(|line| match line.split_once(":") {
            Some((id, dna)) => (id.parse().expect("failed to parse id"), dna.to_string()),
            _ => panic!("invalid input"),
        })
        .collect::<Vec<(usize, String)>>()
}

fn find_parents((id, dna): &Person, everyone: &Vec<Person>) -> Option<Vec<Person>> {
    let candidates = everyone
        .iter()
        .filter(|(other_id, _)| other_id != id)
        .cloned()
        .collect::<Vec<Person>>();

    let combinations = candidates
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            candidates[i..candidates.len()]
                .iter()
                .map(|b| (a.clone(), b.clone()))
        })
        .collect::<Vec<(Person, Person)>>();

    'next_candidate: for ((id_a, dna_a), (id_b, dna_b)) in combinations {
        let chars = dna_a
            .chars()
            .zip(dna_b.chars())
            .collect::<Vec<(char, char)>>();

        for (c, (char_a, char_b)) in dna.chars().zip(chars) {
            if c != char_a && c != char_b {
                continue 'next_candidate;
            }
        }

        // found the child
        return Some(Vec::from([(id_a, dna_a), (id_b, dna_b)]));
    }

    None
}

fn find_child(people: &Vec<Person>) -> Option<(Person, Vec<Person>)> {
    for child in people {
        if let Some(parents) = find_parents(&child, &people) {
            return Some((child.clone(), parents));
        }
    }

    None
}

fn compute_similarity((_, dna): (usize, String), parents: &Vec<(usize, String)>) -> usize {
    parents
        .iter()
        .map(|(_, other_dna)| {
            dna.chars()
                .zip(other_dna.chars())
                .filter(|(c1, c2)| c1 == c2)
                .count()
        })
        .fold(1, |acc, count| acc * count)
}

fn compute_similarities(everyone: &Vec<Person>) -> usize {
    dbg!(
        everyone
            .iter()
            .map(|child| match find_parents(child, &everyone) {
                Some(parents) => Some((child.clone(), parents)),
                _ => None,
            })
            .collect::<Vec<Option<(Person, Vec<Person>)>>>()
    );

    everyone
        .iter()
        .map(|child| match find_parents(child, &everyone) {
            Some(parents) => Some((child.clone(), parents)),
            _ => None,
        })
        .filter_map(|p| p)
        .collect::<Vec<(Person, Vec<Person>)>>()
        .iter()
        .map(|(child, parents)| compute_similarity(child.clone(), &parents))
        .sum()
}

fn build_graph(everyone: &Vec<Person>) -> HashMap<usize, HashSet<usize>> {
    let mut relations: HashMap<usize, HashSet<usize>> = HashMap::new();

    for person in everyone {
        if let Some(p) = find_parents(person, &everyone) {
            let (id, _) = person;

            relations
                .entry(*id)
                .or_default()
                .extend(p.iter().map(|(id, _)| id));

            for (pid, _) in p {
                relations.entry(pid).or_default().extend([id]);
            }
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
            let (child, parents) = find_child(&people).expect("failed to find a valid child");
            compute_similarity(child, &parents)
        }
        Part::Part2 => compute_similarities(&people),
        Part::Part3 => {
            let graph = build_graph(&people);
            find_biggest_family(&graph)
        }
    };

    dbg!(solution);
}
