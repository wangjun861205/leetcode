struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort();
        let mut presum: Vec<(i32, i32)> = people.iter().map(|v| (v[0], 0)).collect();
        presum.dedup();
        let mut queue = Vec::new();
        let head_index = people
            .iter()
            .enumerate()
            .filter(|(_, v)| v[1] == 0)
            .min_by_key(|(_, v)| v[0])
            .map(|(i, _)| i)
            .unwrap();
        let head = people.remove(head_index);
        for i in 0..presum.len() {
            if presum[i].0 <= head[0] {
                presum[i].1 += 1;
            }
        }
        queue.push(head);
        'outer: while !people.is_empty() {
            for i in 0..people.len() {
                let p = people[i].clone();
                let count = presum[presum.binary_search_by(|v| v.0.cmp(&p[0])).unwrap()].1;
                if count == p[1] {
                    people.remove(i);
                    for i in 0..presum.len() {
                        if presum[i].0 <= p[0] {
                            presum[i].1 += 1;
                        }
                    }
                    queue.push(p);
                    continue 'outer;
                }
            }
        }
        queue
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ])
    );
}
