There is an m x n grid, where (0, 0) is the top-left cell and (m - 1, n - 1) is the bottom-right cell. You are given an integer array startPos where startPos = [startrow, startcol] indicates that initially, a robot is at the cell (startrow, startcol). You are also given an integer array homePos where homePos = [homerow, homecol] indicates that its home is at the cell (homerow, homecol).

The robot needs to go to its home. It can move one cell in four directions: left, right, up, or down, and it can not move outside the boundary. Every move incurs some cost. You are further given two 0-indexed integer arrays: rowCosts of length m and colCosts of length n.

If the robot moves up or down into a cell whose row is r, then this move costs rowCosts[r].
If the robot moves left or right into a cell whose column is c, then this move costs colCosts[c].
Return the minimum total cost for this robot to return home.

Example 1:

![](https://assets.leetcode.com/uploads/2021/10/11/eg-1.png)

> Input: startPos = [1, 0], homePos = [2, 3], rowCosts = [5, 4, 3], colCosts = [8, 2, 6, 7]  
> Output: 18

Explanation: One optimal path is that:  
Starting from (1, 0)  
-> It goes down to (2, 0). This move costs rowCosts[2] = 3.  
-> It goes right to (2, 1). This move costs colCosts[1] = 2.  
-> It goes right to (2, 2). This move costs colCosts[2] = 6.  
-> It goes right to (2, 3). This move costs colCosts[3] = 7.  
The total cost is 3 + 2 + 6 + 7 = 18

Example 2:

> Input: startPos = [0, 0], homePos = [0, 0], rowCosts = [5], colCosts = [26]  
> Output: 0

Explanation: The robot is already at its home. Since no moves occur, the total cost is 0.

Constraints:

- m == rowCosts.length
- n == colCosts.length
- 1 <= m, n <= 105
- 0 <= rowCosts[r], colCosts[c] <= 104
- startPos.length == 2
- homePos.length == 2
- 0 <= startrow, homerow < m
- 0 <= startcol, homecol < n

---

这题有点脑筋急转弯的意思，细心的同学看过数量级之后就应该知道别管是 BFS 还是 DFS 都会超时的。那应该怎么解呢？其实我也是看到别人的解析之后才想明白。先说结论， 机器人回家的每条最短路径所需的消耗都是一样的。因为`rowCosts`和`colCosts`都规定的是从别的行或者列走入这一行或列的消耗， 举个例子， 我从 0 行走到 1 行， 此时不论我在那一列，我的消耗都是 rowCosts[1], 同理， 如果我从 0 列走到 1 列, 无论我在哪一行， 消耗都是 colCosts[1]。所以当前位置定了， 家的位置定了， 那我们要走过的行和列也就定下来了， 最短路径就是每个行和列都只进入一次， 所以最短路径就是行直走，列也直走， 只要不绕路， 所有走法都是最短路径。

---

```rust
impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let (mut start_row, mut start_col) = (start_pos[0], start_pos[1]);
        let (home_row, home_col) = (home_pos[0], home_pos[1]);
        let mut ans = 0;
        while start_row != home_row {
            start_row += (home_row - start_row) / (home_row - start_row).abs();
            ans += row_costs[start_row as usize];
        }
        while start_col != home_col {
            start_col += (home_col - start_col) / (home_col - start_col).abs();
            ans += col_costs[start_col as usize];
        }
        ans
    }
}
```
