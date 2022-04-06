struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut to_a = costs[..costs.len() / 2].to_vec();
        let mut to_b = costs[costs.len() / 2..].to_vec();
        for i in 0..costs.len() / 2 {
            for j in 0..costs.len() / 2 {
                let a = to_a[i].clone();
                let b = to_b[j].clone();
                if a[1] + b[0] < a[0] + b[1] {
                    to_a[i] = b;
                    to_b[j] = a;
                }
            }
        }
        to_a.into_iter().map(|v| v[0]).sum::<i32>() + to_b.into_iter().map(|v| v[1]).sum::<i32>()
    }
}

fn main() {
    println!(
        "{}",
        Solution::two_city_sched_cost(vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42]
        ])
    );
}
