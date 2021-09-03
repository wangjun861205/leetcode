struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegrees = vec![0; num_courses as usize];
        let mut rel = vec![Vec::new(); num_courses as usize];
        for p in &prerequisites {
            indegrees[p[0] as usize] += 1;
            rel[p[1] as usize].push(p[0])
        }
        let mut ans = Vec::new();
        loop {
            let mut oped = false;
            for i in 0..indegrees.len() {
                if indegrees[i] == 0 {
                    oped = true;
                    ans.push(i as i32);
                    indegrees[i] = -1;
                    for r in &rel[i] {
                        indegrees[*r as usize] -= 1;
                    }
                }
            }
            if !oped {
                if indegrees.iter().any(|v| v != &-1) {
                    return Vec::new();
                } else {
                    return ans;
                }
            }
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
    );
}
