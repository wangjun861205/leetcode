struct Solution;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        if group_size == 1 {
            return true;
        }
        hand.sort();
        let mut stacks: Vec<Vec<i32>> = vec![Vec::new(); hand.len() / group_size as usize];
        let mut idx = 0;
        for v in hand {
            if stacks[idx].len() == 0 {
                stacks[idx].push(v);
                continue;
            }
            if v == *stacks[idx].last().unwrap() {
                if idx + 1 == stacks.len() {
                    return false;
                } else {
                    if stacks[idx + 1].len() == 0 {
                        stacks[idx + 1].push(v);
                        idx += 1;
                    } else {
                        if v - *stacks[idx + 1].last().unwrap() != 1 {
                            return false;
                        } else {
                            stacks[idx + 1].push(v);
                            idx += 1;
                        }
                    }
                }
            } else {
                if v - *stacks[0].last().unwrap() != 1 {
                    return false;
                } else {
                    stacks[0].push(v);
                    if stacks[0].len() == group_size as usize {
                        stacks.remove(0);
                    }
                    idx = 0;
                }
            }
        }
        stacks.is_empty()
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
    );
}
