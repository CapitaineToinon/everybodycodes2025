use common::{Part, parse_args};
use std::{
    cmp::{max, min},
    f64,
};

const FLOAT_PRECISION: f64 = 1e-6;

type Nail = usize;
type Thread = (Nail, Nail);
type Point = (f64, f64);
type Line = (Point, Point);

fn parse(input: &String) -> Vec<Thread> {
    let ordered_nails = input
        .trim()
        .split(",")
        .map(|n| n.trim().parse().expect("invalid input"))
        .collect::<Vec<Nail>>();

    to_threads(&ordered_nails)
}

fn to_threads(input: &Vec<Nail>) -> Vec<Thread> {
    input
        .windows(2)
        .collect::<Vec<&[usize]>>()
        .iter()
        .map(|slice| match slice {
            &[a, b] => (*a, *b),
            _ => panic!("invalid window"),
        })
        .collect::<Vec<Thread>>()
}

// Converts a nail's 1-indexed index to its (y,x) floating point coordinate.
// Index 1 gives coordinate (-1, 0) and cycles to the right based on the total
// amount of nails on the circle.
fn coordinates(nail: Nail, total: usize) -> Point {
    let angle: f64 = (2.0 * f64::consts::PI / (total as f64)) * ((nail - 1) as f64);
    (f64::sin(angle), -f64::cos(angle))
}

fn count_centers(threads: &Vec<Thread>, nails: usize) -> usize {
    threads
        .iter()
        .filter(|(a, b)| max(a, b) - min(a, b) == nails / 2)
        .count()
}

fn count_knots(threads: &Vec<Thread>, nails: usize) -> usize {
    threads.iter().enumerate().fold(0, |acc, (i, &thread_a)| {
        acc + threads.as_slice()[0..i]
            .iter()
            .to_owned()
            .filter(|&thread_b| intersects(thread_a, *thread_b, nails))
            .count()
    })
}

fn find_max_cuts(threads: &Vec<Thread>, knots: usize) -> usize {
    (0..knots)
        .flat_map(|i| (i..knots).map(move |j| (i + 1, j + 1)))
        .filter(|(i, j)| i != j)
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .map(|(i, j)| {
            let mut cuts = threads
                .iter()
                .filter(|&other| intersects((i, j), *other, knots))
                .count();

            if threads.contains(&(i, j)) || threads.contains(&(j, i)) {
                cuts += 1;
            }

            cuts
        })
        .max()
        .expect("knots has be to greated than 1")
}

fn get_intersection((p1, p2): Line, (p3, p4): Line) -> Option<Point> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let (x4, y4) = p4;

    let det = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if det.abs() < 0.0 + FLOAT_PRECISION {
        // if determinant is zero it means the two
        // lines are perpendicular and never intersect
        return None;
    }

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / det;
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / det;

    Some((px, py))
}

fn intersects((nail_a1, nail_a2): Thread, (nail_b1, nail_b2): Thread, knots: usize) -> bool {
    let point_a1 = coordinates(nail_a1, knots);
    let point_a2 = coordinates(nail_a2, knots);
    let point_b1 = coordinates(nail_b1, knots);
    let point_b2 = coordinates(nail_b2, knots);

    match get_intersection((point_a1, point_a2), (point_b1, point_b2)) {
        // if norm is 1.0, it means the intersection is on the unit circle,
        // meaning the two lines share a point and therefore don't intersect
        Some(p) => norm(p) < (1.0 - FLOAT_PRECISION),
        None => false,
    }
}

// L2 norm
fn norm((x, y): Point) -> f64 {
    f64::sqrt(x.powf(2.0) + y.powf(2.0))
}

fn main() {
    let args = parse_args();
    let lines = parse(&args.input);

    let solution = match args.part {
        Part::Part1 => count_centers(&lines, 32),
        Part::Part2 => count_knots(&lines, 256),
        Part::Part3 => find_max_cuts(&lines, 256),
    };

    println!("{}", solution)
}
