struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Condition(i32, i32, i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rating(i32, i32);

impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        let mut vf_heap: BinaryHeap<Reverse<Condition>> = BinaryHeap::new();
        let mut all_heap: BinaryHeap<Reverse<Condition>> = BinaryHeap::new();
        for r in &restaurants {
            let cond = Reverse(Condition(r[3], r[4], r[1], r[0]));
            if r[2] == 0 {
                all_heap.push(cond);
            } else {
                all_heap.push(cond.clone());
                vf_heap.push(cond.clone());
            }
        }
        let mut h;
        if vegan_friendly == 1 {
            h = vf_heap;
        } else {
            h = all_heap;
        }
        let mut rating_heap: BinaryHeap<Rating> = BinaryHeap::new();
        while let Some(Reverse(c)) = h.pop() {
            if c.0 <= max_price {
                if c.1 <= max_distance {
                    rating_heap.push(Rating(c.2, c.3));
                }
            } else {
                break;
            }
        }
        let mut ans = Vec::new();
        while let Some(r) = rating_heap.pop() {
            ans.push(r.1);
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::filter_restaurants(
            vec![vec![1, 4, 1, 40, 10], vec![2, 8, 0, 50, 5], vec![3, 8, 1, 30, 4], vec![4, 10, 0, 10, 3], vec![5, 1, 1, 15, 1]],
            0,
            50,
            10
        )
    );
}
