struct Solution;

impl Solution {
    fn dfs(
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        pos: (i32, i32),
        from: (i32, i32),
        c: char,
    ) -> bool {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 == grid.len() as i32
            || pos.1 == grid[0].len() as i32
            || grid[pos.0 as usize][pos.1 as usize] != c
        {
            return false;
        }

        if visited[pos.0 as usize][pos.1 as usize] && grid[pos.0 as usize][pos.1 as usize] == c {
            return true;
        }
        visited[pos.0 as usize][pos.1 as usize] = true;
        let up = if (pos.0 - 1, pos.1) != from {
            (pos.0 - 1, pos.1)
        } else {
            (-1, -1)
        };
        if Solution::dfs(grid, visited, up, pos, c) {
            return true;
        }
        let down = if (pos.0 + 1, pos.1) != from {
            (pos.0 + 1, pos.1)
        } else {
            (-1, -1)
        };
        if Solution::dfs(grid, visited, down, pos, c) {
            return true;
        }
        let left = if (pos.0, pos.1 - 1) != from {
            (pos.0, pos.1 - 1)
        } else {
            (-1, -1)
        };
        if Solution::dfs(grid, visited, left, pos, c) {
            return true;
        }
        let right = if (pos.0, pos.1 + 1) != from {
            (pos.0, pos.1 + 1)
        } else {
            (-1, -1)
        };
        if Solution::dfs(grid, visited, right, pos, c) {
            return true;
        }
        false
    }
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut visited: Vec<Vec<bool>> = grid
            .iter()
            .map(|l| l.iter().map(|_| false).collect())
            .collect();
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if !visited[r][c] {
                    if Solution::dfs(
                        &grid,
                        &mut visited,
                        (r as i32, c as i32),
                        (-1, -1),
                        grid[r][c],
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a']
        ])
    );
}
