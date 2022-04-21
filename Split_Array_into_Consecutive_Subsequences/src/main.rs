struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Subsequence(i32, i32);

impl Ord for Subsequence {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            return (other.1 - other.0).cmp(&(self.1 - self.0));
        }
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Subsequence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut heap: BinaryHeap<Subsequence> = BinaryHeap::new();
        'outer: for num in nums {
            if heap.is_empty() {
                heap.push(Subsequence(num, num));
                continue;
            }
            while let Some(Subsequence(start, end)) = heap.pop() {
                if num > end + 1 {
                    if end - start < 2 {
                        return false;
                    }
                    continue;
                } else if num == end {
                    heap.push(Subsequence(start, end));
                    break;
                } else {
                    heap.push(Subsequence(start, end + 1));
                    continue 'outer;
                }
            }
            heap.push(Subsequence(num, num));
        }
        if heap
            .into_iter()
            .any(|Subsequence(start, end)| end - start < 2)
        {
            return false;
        }
        true
    }
}

fn main() {
    println!("{}", Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
}
