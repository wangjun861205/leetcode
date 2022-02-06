struct Solution;

impl Solution {
    fn find(parents: &mut Vec<i32>, n: i32) -> i32 {
        if parents[n as usize] == n {
            return n;
        }
        let parent = Solution::find(parents, parents[n as usize]);
        parents[n as usize] = parent;
        parent
    }
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let mut parents: Vec<i32> = (0..n).into_iter().collect();
        for c in &connections {
            let p1 = Solution::find(&mut parents, c[0]);
            let p2 = Solution::find(&mut parents, c[1]);
            parents[p2 as usize] = p1;
        }
        for i in 0..n {
            Solution::find(&mut parents, i);
        }
        parents
            .into_iter()
            .enumerate()
            .filter(|(i, v)| *v == *i as i32)
            .count() as i32
            - 1
    }
}

fn main() {
    println!(
        "{}",
        Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
        )
    );
}
