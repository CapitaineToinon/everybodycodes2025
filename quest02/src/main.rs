use core::fmt;
use std::ops::{self};

#[derive(Debug, Clone, Copy)]
struct Complex {
    x: i64,
    y: i64,
}

impl Complex {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl ops::Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

fn part_1(a: Complex) -> Complex {
    let mut r = Complex::new(0, 0);

    for _ in 0..3 {
        r = r * r;
        r = r / Complex::new(10, 10);
        r = r + a;
    }

    r
}

fn part_2(origin: Complex, step: usize) -> usize {
    let opposite = origin + Complex::new(1_000, 1_000);
    let mut total = 0;

    for x in (origin.x..=opposite.x).step_by(step) {
        for y in (origin.y..=opposite.y).step_by(step) {
            if is_engraved(Complex::new(x, y)) {
                total += 1;
            }
        }
    }

    total
}

fn is_engraved(a: Complex) -> bool {
    let mut r = Complex::new(0, 0);

    for _ in 0..100 {
        r = r * r;
        r = r / Complex::new(100_000, 100_000);
        r = r + a;

        if r.x < -1_000_000 || r.x > 1_000_000 || r.y < -1_000_000 || r.y > 1_000_000 {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", part_1(Complex::new(161, 53)));
    println!("{}", part_2(Complex::new(-4591, 67892), 10));
    println!("{}", part_2(Complex::new(-4591, 67892), 1));
}
