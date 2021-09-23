struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, mut rides: Vec<Vec<i32>>) -> i64 {
        let mut earns = vec![0_i64; n as usize];
        rides.sort_by_key(|v| (v[1], v[0], v[2]));
        for i in 0..n as usize {
            if i > 0 {
                earns[i] = earns[i - 1];
            }
            while !rides.is_empty() && rides.first().clone().unwrap()[1] == i as i32 + 1 {
                let r = rides.remove(0);
                let earn = earns[r[0] as usize - 1] + r[1] as i64 - r[0] as i64 + r[2] as i64;
                earns[r[1] as usize - 1] = earns[r[1] as usize - 1].max(earn);
            }
        }
        for r in rides {
            let prev = *earns[0..r[0] as usize].into_iter().max().unwrap();
            let earn = prev + r[1] as i64 - r[0] as i64 + r[2] as i64;
            earns[r[1] as usize - 1] = earns[r[1] as usize - 1].max(earn);
        }
        earns.into_iter().max().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_taxi_earnings(
            10,
            vec![
                vec![9, 10, 2],
                vec![4, 5, 6],
                vec![6, 8, 1],
                vec![1, 5, 5],
                vec![4, 9, 5],
                vec![1, 6, 5],
                vec![4, 8, 3],
                vec![4, 7, 10],
                vec![1, 9, 8],
                vec![2, 3, 5]
            ]
        )
    );
}
