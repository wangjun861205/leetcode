struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        for i in (1..stations.len()).rev() {
            stations[i][0] = stations[i][0] - stations[i - 1][0];
        }
        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        let mut remain = target;
        let mut fuel = start_fuel;
        let mut ans = 0;
        for s in stations {
            remain -= s[0];
            fuel -= s[0];
            if fuel < 0 {
                while !queue.is_empty() {
                    fuel += queue.pop().unwrap();
                    ans += 1;
                    if fuel >= 0 {
                        break;
                    }
                }
                if fuel < 0 {
                    return -1;
                }
            }
            queue.push(s[1]);
        }
        if fuel < remain {
            while !queue.is_empty() {
                fuel += queue.pop().unwrap();
                ans += 1;
                if fuel >= remain {
                    return ans;
                }
            }
            return -1;
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_refuel_stops(100, 25, vec![vec![25, 25], vec![50, 25], vec![75, 25]])
    );
}
