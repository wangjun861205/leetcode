A company is planning to interview 2n people. Given the array costs where costs[i] = [aCosti, bCosti], the cost of flying the ith person to city a is aCosti, and the cost of flying the ith person to city b is bCosti.

Return the minimum cost to fly every person to a city such that exactly n people arrive in each city.

Example 1:

> Input: costs = [[10,20],[30,200],[400,50],[30,20]]  
> Output: 110

Explanation:  
The first person goes to city A for a cost of 10.  
The second person goes to city A for a cost of 30.  
The third person goes to city B for a cost of 50.  
The fourth person goes to city B for a cost of 20.

The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.

Example 2:

> Input: costs = [[259,770],[448,54],[926,667],[184,139],[840,118],[577,469]]  
> Output: 1859

Example 3:

> Input: costs = [[515,563],[451,713],[537,709],[343,819],[855,779],[457,60],[650,359],[631,42]]  
> Output: 3086

Constraints:

- 2 \* n == costs.length
- 2 <= costs.length <= 100
- costs.length is even.
- 1 <= aCosti, bCosti <= 1000

---

这题一开始觉得应该用 DP 来做， 结果超时了。然后自己想了另一种解法， 感觉应该算是贪婪算法， 不知道是不是， 记录在这，大家看一下:

1. 先把`costs`均分成两个数组`to_a`(到城市 A 进行面试)和`to_b`(到城市 B 进行面试)
2. 遍历`to_a`将每一个元素与`to_b`中的每个元素进行对比， 如果交换之后可以减少成本则进行交换

每一步的最优解积累成最终的整体最优

---

```rust
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
```
