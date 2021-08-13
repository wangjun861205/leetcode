struct Solution;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Groups {
    length: i32,
    groups: Vec<(char, i32)>,
}

impl Groups {
    fn new(s: impl Into<String>) -> Self {
        let groups = s.into().chars().fold(Vec::new(), |mut l, v| {
            if l.is_empty() {
                l.push((v, 1));
            } else {
                let last = l.last().unwrap().clone();
                if last.0 == v {
                    l.last_mut().unwrap().1 += 1;
                } else {
                    l.push((v, 1));
                }
            }
            l
        });
        let length = Solution::count(&groups);
        Self { length, groups }
    }

    fn repr_len(&self) -> i32 {
        self.length
    }

    fn group_len(&self) -> usize {
        self.groups.len()
    }

    fn remove(&mut self, i: usize, n: i32) {
        let ori = self.groups[i].1;
        let new = ori - n;
        self.length -= Solution::get_diff(ori, new);
        self.groups[i].1 = new;
        if new == 0 {
            if i != 0 && i != self.groups.len() - 1 && self.groups[i - 1].0 == self.groups[i + 1].0
            {
                let left_ori = self.groups[i - 1].1;
                let left_new = self.groups[i - 1].1 + self.groups[i + 1].1;
                let right_ori = self.groups[i + 1].1;
                let right_new = 0;
                let diff = Solution::get_diff(left_ori, left_new)
                    + Solution::get_diff(right_ori, right_new);
                self.length -= diff;
                self.groups[i - 1].1 = left_new;
                self.groups[i + 1].1 = 0;
                self.groups.remove(i + 1);
                self.groups.remove(i);
            } else {
                self.groups.remove(i);
            }
        }
    }
}

impl Solution {
    fn count(groups: &Vec<(char, i32)>) -> i32 {
        groups
            .iter()
            .map(|(_, n)| {
                if n == &0 {
                    0
                } else if n == &1 {
                    1
                } else if n < &10 {
                    2
                } else if n < &99 {
                    3
                } else {
                    4
                }
            })
            .sum()
    }

    fn get_diff(ori: i32, new: i32) -> i32 {
        if new > ori {
            return -Solution::get_diff(new, ori);
        }
        if new == 0 {
            if ori == 1 {
                return 1;
            } else if ori < 10 {
                return 2;
            } else if ori < 100 {
                return 3;
            } else {
                return 4;
            }
        } else if new == 1 {
            if ori < 10 {
                return 1;
            } else if ori < 100 {
                return 2;
            } else {
                return 3;
            }
        } else if new < 10 {
            if ori < 10 {
                return 0;
            } else if ori < 100 {
                return 1;
            } else {
                return 2;
            }
        } else if new < 100 {
            if ori < 100 {
                return 0;
            } else if ori == 100 {
                return 1;
            }
        }
        0
    }

    fn dp(groups: Groups, k: i32, cache: &mut HashMap<Groups, i32>) -> i32 {
        let mut ans = groups.repr_len();
        for i in 0..groups.group_len() {
            for n in 1..=k.min(groups.groups[i].1) {
                let mut next = groups.clone();
                next.remove(i, n);
                let res = if let Some(c) = cache.get(&next) {
                    *c
                } else {
                    Solution::dp(next, k - n, cache)
                };
                ans = ans.min(res);
            }
        }
        cache.insert(groups, ans);
        ans
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let groups = Groups::new(s);
        Solution::dp(groups, k, &mut HashMap::new())
    }
}

fn main() {
    // println!(
    //     "{}",
    //     Solution::get_length_of_optimal_compression("aabaabbcbbbaccc".to_owned(), 6)
    // );
    let mut groups = Groups::new("aabaabbcbbbaccc");
    println!("{:?}", groups);
    groups.remove(1, 1);
    println!("{:?}", groups);
    groups.remove(2, 1);
    println!("{:?}", groups);
    groups.remove(2, 1);
    println!("{:?}", groups);
    groups.remove(2, 3);
    println!("{:?}", groups);
}
