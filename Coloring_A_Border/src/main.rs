struct Solution;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
        // 起始格子的原始颜色, 用于后期恢复处于中间位置的格子时使用
        let ori_color = grid[r0 as usize][c0 as usize];
        let mut queue: Vec<(usize, usize)> = vec![(r0 as usize, c0 as usize)];
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (row, col) = queue.remove(0);
                // 相邻的处于同一connected component的格子数量
                let mut count = 0;
                if row > 0 {
                    // 如果是同一颜色的证明还没有走过， 推入队列, 计数加1
                    if grid[row - 1][col] == ori_color {
                        queue.push((row - 1, col));
                        count += 1;
                    }
                    // 如果是已经走过的格子， 则只增加计数就好
                    if grid[row - 1][col] == 1001 || grid[row - 1][col] == 1002 {
                        count += 1;
                    }
                }
                if row < grid.len() - 1 {
                    if grid[row + 1][col] == ori_color {
                        queue.push((row + 1, col));
                        count += 1;
                    }
                    if grid[row + 1][col] == 1001 || grid[row + 1][col] == 1002 {
                        count += 1;
                    }
                }
                if col > 0 {
                    if grid[row][col - 1] == ori_color {
                        queue.push((row, col - 1));
                        count += 1;
                    }
                    if grid[row][col - 1] == 1001 || grid[row][col - 1] == 1002 {
                        count += 1;
                    }
                }
                if col < grid[0].len() - 1 {
                    if grid[row][col + 1] == ori_color {
                        queue.push((row, col + 1));
                        count += 1;
                    }
                    if grid[row][col + 1] == 1001 || grid[row][col + 1] == 1002 {
                        count += 1
                    }
                }
                if count < 4 {
                    grid[row][col] = 1002;
                } else {
                    grid[row][col] = 1001;
                }
            }
        }
        // 遍历grid, 将值为1001的格子（处于中心的格子)填充回原来的颜色， 将值为1002的格子填充为目标颜色
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| {
                if v == &1001 {
                    *v = ori_color;
                } else if v == &1002 {
                    *v = color;
                }
            });
        });
        grid
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::color_border(
            vec![
                vec![1, 2, 1, 2, 1, 2],
                vec![2, 2, 2, 2, 1, 2],
                vec![1, 2, 2, 2, 1, 2]
            ],
            1,
            3,
            1
        )
    );
}
