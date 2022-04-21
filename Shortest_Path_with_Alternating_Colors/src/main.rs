struct Solution;

impl Solution {
    fn bfs(
        n: i32,
        red_edges: &mut Vec<Vec<usize>>,
        blue_edges: &mut Vec<Vec<usize>>,
        mut color: usize,
    ) -> Vec<i32> {
        let mut ans = vec![i32::MAX; n as usize];
        ans[0] = 0;
        let mut visited = vec![vec![vec![false, false]; n as usize]; n as usize];
        let mut steps = 0;
        let mut stack = vec![0usize];
        while !stack.is_empty() {
            let mut new_stack = Vec::new();
            for n in stack {
                ans[n] = ans[n].min(steps);
                if color == 0 {
                    for &next in &blue_edges[n] {
                        if !visited[n][next][color] {
                            new_stack.push(next);
                            visited[n][next][color] = true;
                        }
                    }
                } else {
                    for &next in &red_edges[n] {
                        if !visited[n][next][color] {
                            new_stack.push(next);
                            visited[n][next][color] = true;
                        }
                    }
                }
            }
            steps += 1;
            stack = new_stack;
            color = (color as i32 - 1).abs() as usize;
        }
        ans
    }

    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut red_edges: Vec<Vec<usize>> =
            red_edges
                .into_iter()
                .fold(vec![Vec::new(); n as usize], |mut l, p| {
                    l[p[0] as usize].push(p[1] as usize);
                    l
                });
        let mut blue_edges: Vec<Vec<usize>> =
            blue_edges
                .into_iter()
                .fold(vec![Vec::new(); n as usize], |mut l, p| {
                    l[p[0] as usize].push(p[1] as usize);
                    l
                });
        let red_first = Solution::bfs(n, &mut red_edges, &mut blue_edges, 0);
        let blue_first = Solution::bfs(n, &mut red_edges, &mut blue_edges, 1);

        red_first
            .into_iter()
            .zip(blue_first)
            .map(|(i, j)| i.min(j))
            .map(|v| if v == i32::MAX { -1 } else { v })
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_alternating_paths(
            5,
            vec![
                vec![2, 2],
                vec![0, 1],
                vec![0, 3],
                vec![0, 0],
                vec![0, 4],
                vec![2, 1],
                vec![2, 0],
                vec![1, 4],
                vec![3, 4]
            ],
            vec![vec![1, 3], vec![0, 0], vec![0, 3], vec![4, 2], vec![1, 0]]
        )
    );
}
