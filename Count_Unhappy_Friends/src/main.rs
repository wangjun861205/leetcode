struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let distances: Vec<Vec<i32>> = preferences
            .iter()
            .map(|l| {
                l.iter()
                    .enumerate()
                    .fold(vec![-1; n as usize], |mut ll, (i, &v)| {
                        ll[v as usize] = i as i32;
                        ll
                    })
            })
            .collect();
        let mut ps = vec![0; n as usize];
        for p in pairs {
            ps[p[0] as usize] = p[1];
            ps[p[1] as usize] = p[0];
        }
        let mut count = 0;
        'outer: for i in 0..ps.len() {
            let x = i as i32;
            let y = ps[i];
            for &u in &preferences[x as usize] {
                if u == x {
                    continue 'outer;
                }
                let v = ps[u as usize];
                let d1 = distances[x as usize][u as usize];
                let d2 = distances[x as usize][y as usize];
                let d3 = distances[u as usize][x as usize];
                let d4 = distances[u as usize][v as usize];
                if d1 < d2 && d3 < d4 {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::unhappy_friends(
            4,
            vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
            vec![vec![0, 1], vec![2, 3]]
        )
    );
}
