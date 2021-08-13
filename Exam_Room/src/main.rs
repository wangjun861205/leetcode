use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Eq, PartialEq, Debug)]
struct Range(i32, i32);

impl Range {
    fn max_dist(&self) -> i32 {
        if self.0 < 0 || self.1 < 0 {
            self.1.abs() - self.0.abs()
        } else {
            (self.1.abs() - self.0.abs()) / 2
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let dist1 = self.max_dist();
        let dist2 = other.max_dist();
        if dist1 < dist2 {
            return Ordering::Less;
        } else if dist1 > dist2 {
            return Ordering::Greater;
        } else {
            if self.0.abs() < other.0.abs() {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }
}

struct ExamRoom {
    ranges: Vec<Range>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            ranges: vec![Range(-1, -n)],
        }
    }

    fn seat(&mut self) -> i32 {
        let max_idx = self
            .ranges
            .iter()
            .enumerate()
            .max_by_key(|(_, r)| *r)
            .map(|(i, _)| i)
            .unwrap();
        let Range(s, e) = self.ranges[max_idx];
        if s < 0 {
            self.ranges[max_idx].0 = -s;
            return -s - 1;
        }
        if e < 0 {
            self.ranges[max_idx].1 = -e;
            return -e - 1;
        }
        let mid = s + (e - s) / 2;
        self.ranges[max_idx].1 = mid;
        self.ranges.insert(max_idx + 1, Range(mid, e));
        mid - 1
    }

    fn leave(&mut self, p: i32) {
        if p == 0 {
            self.ranges[0].0 = -1;
            return;
        }
        if p == self.ranges.last().unwrap().1 - 1 {
            self.ranges.last_mut().unwrap().1 = -self.ranges.last_mut().unwrap().1;
            return;
        }
        let pos = self
            .ranges
            .iter()
            .position(|&Range(s, e)| s <= p + 1 && e >= p + 1)
            .unwrap();
        self.ranges[pos].1 = self.ranges[pos + 1].1;
        self.ranges.remove(pos + 1);
    }
}

fn main() {
    let mut room = ExamRoom::new(10);
    for _ in 0..4 {
        println!("{}", room.seat());
    }
    room.leave(4);
    println!("{}", room.seat());
    println!("{:?}", room.ranges)
}
