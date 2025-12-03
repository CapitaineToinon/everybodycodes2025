use std::cmp::Ordering;

#[derive(Debug)]
pub struct Sword {
    pub id: i64,
    spine: i64,
    left: Option<i64>,
    right: Option<i64>,
    next: Option<Box<Sword>>,
}

impl Clone for Sword {
    fn clone(&self) -> Self {
        Sword {
            id: self.id,
            spine: self.spine,
            left: self.left,
            right: self.right,
            next: match &self.next {
                Some(n) => Some(Box::new(*n.clone())),
                _ => None,
            },
        }
    }
}

impl Sword {
    pub fn new(id: i64, spine: i64) -> Self {
        Sword {
            id,
            spine,
            left: None,
            right: None,
            next: None,
        }
    }

    pub fn from_input(id: i64, steps: Vec<i64>) -> Self {
        match steps.as_slice() {
            [f, tail @ ..] => Sword::new(id, *f).process_steps(tail),
            _ => panic!("steps cannot be empty"),
        }
    }

    fn add_left(&self, left: i64) -> Self {
        Sword {
            left: Some(left),
            id: self.id,
            spine: self.spine,
            right: self.right,
            next: self.next.clone(),
        }
    }

    fn add_right(&self, right: i64) -> Self {
        Sword {
            right: Some(right),
            id: self.id,
            spine: self.spine,
            left: self.left,
            next: self.next.clone(),
        }
    }

    fn add_next(&self, next: Self) -> Self {
        Sword {
            next: Some(Box::new(next)),
            id: self.id,
            spine: self.spine,
            left: self.left,
            right: self.right,
        }
    }

    fn process_steps(&self, steps: &[i64]) -> Self {
        steps
            .iter()
            .fold(self.clone(), |s, step| s.process_step(*step))
    }

    fn process_step(&self, step: i64) -> Sword {
        if let None = self.left
            && step < self.spine
        {
            return self.add_left(step);
        }

        if let None = self.right
            && step > self.spine
        {
            return self.add_right(step);
        }

        match &self.next {
            None => self.add_next(Sword::new(self.id, step)),
            Some(n) => self.add_next(n.process_step(step)),
        }
    }

    pub fn quality(&self) -> i64 {
        match &self.next {
            Some(n) => (self.spine * 10_i64.pow(n.quality().ilog10() + 1)) + n.quality(),
            _ => self.spine,
        }
    }

    fn level(&self) -> i64 {
        let level = match self.right {
            Some(r) => (self.spine * 10_i64.pow(r.ilog10() + 1)) + r,
            _ => self.spine,
        };

        match self.left {
            Some(l) => (level * 10_i64.pow(level.ilog10() + 1)) + l,
            _ => level,
        }
    }

    pub fn cmp_simple(&self, rhs: &Self) -> Ordering {
        self.quality().cmp(&rhs.quality())
    }

    pub fn cmp_full(&self, rhs: &Self) -> Ordering {
        let quality_cmp = self.cmp_simple(rhs);
        if quality_cmp != Ordering::Equal {
            return quality_cmp;
        }

        let level_cmp = self.level().cmp(&rhs.level());
        if level_cmp != Ordering::Equal {
            return level_cmp;
        }

        match (&self.next, &rhs.next) {
            (Some(a), Some(b)) => a.cmp_full(&b),
            _ => self.id.cmp(&rhs.id),
        }
    }
}

