struct Solution;

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Code(i8, i8, i8, i8);

impl From<String> for Code {
    fn from(s: String) -> Self {
        if s.len() != 4 {
            panic!("invalid code string");
        }
        let chars: Vec<char> = s.chars().collect();
        let mut code = Code(0, 0, 0, 0);
        code.0 = chars[0].to_string().parse().unwrap();
        code.1 = chars[1].to_string().parse().unwrap();
        code.2 = chars[2].to_string().parse().unwrap();
        code.3 = chars[3].to_string().parse().unwrap();
        code
    }
}

impl Code {
    fn add0(&self) -> Self {
        let mut code = self.clone();
        code.0 += 1;
        if code.0 == 10 {
            code.0 = 0
        }
        code
    }
    fn dec0(&self) -> Self {
        let mut code = self.clone();
        code.0 -= 1;
        if code.0 == -1 {
            code.0 = 9;
        }
        code
    }
    fn add1(&self) -> Self {
        let mut code = self.clone();
        code.1 += 1;
        if code.1 == 10 {
            code.1 = 0
        }
        code
    }
    fn dec1(&self) -> Self {
        let mut code = self.clone();
        code.1 -= 1;
        if code.1 == -1 {
            code.1 = 9;
        }
        code
    }
    fn add2(&self) -> Self {
        let mut code = self.clone();
        code.2 += 1;
        if code.2 == 10 {
            code.2 = 0
        }
        code
    }
    fn dec2(&self) -> Self {
        let mut code = self.clone();
        code.2 -= 1;
        if code.2 == -1 {
            code.2 = 9;
        }
        code
    }
    fn add3(&self) -> Self {
        let mut code = self.clone();
        code.3 += 1;
        if code.3 == 10 {
            code.3 = 0
        }
        code
    }
    fn dec3(&self) -> Self {
        let mut code = self.clone();
        code.3 -= 1;
        if code.3 == -1 {
            code.3 = 9;
        }
        code
    }
    fn successors(&self) -> VecDeque<Self> {
        let mut l: VecDeque<Self> = VecDeque::new();
        l.push_back(self.add0());
        l.push_back(self.add1());
        l.push_back(self.add2());
        l.push_back(self.add3());
        l.push_back(self.dec0());
        l.push_back(self.dec1());
        l.push_back(self.dec2());
        l.push_back(self.dec3());
        l
    }
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<Code> = deadends.into_iter().map(|v| Code::from(v)).collect();
        let target = Code::from(target);
        let mut visited: HashSet<Code> = deadends.into_iter().collect();
        let mut steps = 0;
        let mut queue: VecDeque<Code> = vec![Code(0, 0, 0, 0)].into();
        while !queue.is_empty() {
            let mut new_queue: VecDeque<Code> = VecDeque::new();
            for _ in 0..queue.len() {
                let c = queue.remove(0).unwrap();
                if c == target {
                    return steps;
                }
                if visited.contains(&c) {
                    continue;
                }
                visited.insert(c.clone());
                new_queue.append(&mut c.successors());
            }
            steps += 1;
            queue = new_queue;
        }
        -1
    }
}
fn main() {
    println!("{}", Solution::open_lock(vec!["8888".to_owned()], "0009".to_owned()));
}
