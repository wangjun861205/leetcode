struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut points = vec![(src, 0)];
        let mut stops = 0;
        let fs = flights.into_iter().fold(HashMap::new(), |mut m, v| {
            m.entry(v[0]).or_insert(Vec::new()).push((v[1], v[2]));
            m
        });
        let mut m = HashMap::new();
        let mut ans = i32::MAX;
        while !points.is_empty() && stops <= k {
            let mut new_points = Vec::new();
            for (p, c) in points {
                if let Some(l) = fs.get(&p) {
                    for (np, nc) in l {
                        if *np == dst {
                            ans = ans.min(c + *nc);
                            continue;
                        }
                        let entry = m.entry(*np).or_insert(c + *nc);
                        if entry < &mut (c + *nc) {
                            continue;
                        }
                        *entry = c + nc;
                        new_points.push((*np, c + nc));
                    }
                }
            }
            points = new_points;
            stops += 1;
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_cheapest_price(
            17,
            vec![
                vec![0, 12, 28],
                vec![5, 6, 39],
                vec![8, 6, 59],
                vec![13, 15, 7],
                vec![13, 12, 38],
                vec![10, 12, 35],
                vec![15, 3, 23],
                vec![7, 11, 26],
                vec![9, 4, 65],
                vec![10, 2, 38],
                vec![4, 7, 7],
                vec![14, 15, 31],
                vec![2, 12, 44],
                vec![8, 10, 34],
                vec![13, 6, 29],
                vec![5, 14, 89],
                vec![11, 16, 13],
                vec![7, 3, 46],
                vec![10, 15, 19],
                vec![12, 4, 58],
                vec![13, 16, 11],
                vec![16, 4, 76],
                vec![2, 0, 12],
                vec![15, 0, 22],
                vec![16, 12, 13],
                vec![7, 1, 29],
                vec![7, 14, 100],
                vec![16, 1, 14],
                vec![9, 6, 74],
                vec![11, 1, 73],
                vec![2, 11, 60],
                vec![10, 11, 85],
                vec![2, 5, 49],
                vec![3, 4, 17],
                vec![4, 9, 77],
                vec![16, 3, 47],
                vec![15, 6, 78],
                vec![14, 1, 90],
                vec![10, 5, 95],
                vec![1, 11, 30],
                vec![11, 0, 37],
                vec![10, 4, 86],
                vec![0, 8, 57],
                vec![6, 14, 68],
                vec![16, 8, 3],
                vec![13, 0, 65],
                vec![2, 13, 6],
                vec![5, 13, 5],
                vec![8, 11, 31],
                vec![6, 10, 20],
                vec![6, 2, 33],
                vec![9, 1, 3],
                vec![14, 9, 58],
                vec![12, 3, 19],
                vec![11, 2, 74],
                vec![12, 14, 48],
                vec![16, 11, 100],
                vec![3, 12, 38],
                vec![12, 13, 77],
                vec![10, 9, 99],
                vec![15, 13, 98],
                vec![15, 12, 71],
                vec![1, 4, 28],
                vec![7, 0, 83],
                vec![3, 5, 100],
                vec![8, 9, 14],
                vec![15, 11, 57],
                vec![3, 6, 65],
                vec![1, 3, 45],
                vec![14, 7, 74],
                vec![2, 10, 39],
                vec![4, 8, 73],
                vec![13, 5, 77],
                vec![10, 0, 43],
                vec![12, 9, 92],
                vec![8, 2, 26],
                vec![1, 7, 7],
                vec![9, 12, 10],
                vec![13, 11, 64],
                vec![8, 13, 80],
                vec![6, 12, 74],
                vec![9, 7, 35],
                vec![0, 15, 48],
                vec![3, 7, 87],
                vec![16, 9, 42],
                vec![5, 16, 64],
                vec![4, 5, 65],
                vec![15, 14, 70],
                vec![12, 0, 13],
                vec![16, 14, 52],
                vec![3, 10, 80],
                vec![14, 11, 85],
                vec![15, 2, 77],
                vec![4, 11, 19],
                vec![2, 7, 49],
                vec![10, 7, 78],
                vec![14, 6, 84],
                vec![13, 7, 50],
                vec![11, 6, 75],
                vec![5, 10, 46],
                vec![13, 8, 43],
                vec![9, 10, 49],
                vec![7, 12, 64],
                vec![0, 10, 76],
                vec![5, 9, 77],
                vec![8, 3, 28],
                vec![11, 9, 28],
                vec![12, 16, 87],
                vec![12, 6, 24],
                vec![9, 15, 94],
                vec![5, 7, 77],
                vec![4, 10, 18],
                vec![7, 2, 11],
                vec![9, 5, 41]
            ],
            13,
            4,
            13,
        )
    );
}
