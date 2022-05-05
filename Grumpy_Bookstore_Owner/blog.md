There is a bookstore owner that has a store open for n minutes. Every minute, some number of customers enter the store. You are given an integer array customers of length n where customers[i] is the number of the customer that enters the store at the start of the ith minute and all those customers leave after the end of that minute.

On some minutes, the bookstore owner is grumpy. You are given a binary array grumpy where grumpy[i] is 1 if the bookstore owner is grumpy during the ith minute, and is 0 otherwise.

When the bookstore owner is grumpy, the customers of that minute are not satisfied, otherwise, they are satisfied.

The bookstore owner knows a secret technique to keep themselves not grumpy for minutes consecutive minutes, but can only use it once.

Return the maximum number of customers that can be satisfied throughout the day.

Example 1:

> Input: customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3  
> Output: 16

Explanation: The bookstore owner keeps themselves not grumpy for the last 3 minutes.
The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.

Example 2:

> Input: customers = [1], grumpy = [0], minutes = 1  
> Output: 1

Constraints:

- n == customers.length == grumpy.length
- 1 <= minutes <= n <= 2 \* 104
- 0 <= customers[i] <= 1000
- grumpy[i] is either 0 or 1.

---

店主在任意的第`i`分钟克制自己的情绪， 这时候会把整个数组分成三部分`0..i-1`和`i+minutes..length`这两部分是情绪正常发作的区间， `i..i+minutes`是克制情绪的区间， 我们先根据`customers`和`grumpy`来做 prefix sum, 这个 prefix sum 中的每个元素包含两部分， 一个是受情绪影响的累加， 一个是不受情绪影响的累加(克制状态), 有个那三个区间和 prefix sum, 我们就可以很快算出在第`i`分钟进行克制的情况下的满意顾客数量。依次遍历即可

---

```rust
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let length = customers.len();
        let mut presum: Vec<(i32, i32)> = customers
            .into_iter()
            .zip(grumpy)
            .scan((0, 0), |(norm, curb), (c, g)| {
                *norm += c * (g - 1).abs();
                *curb += c;
                Some((*norm, *curb))
            })
            .collect();
        presum.insert(0, (0, 0));
        let mut ans = 0;
        for i in 0..length {
            let mut sum = 0;
            sum += presum[i].0;
            let curb_right_index = i + minutes as usize;
            if curb_right_index < length + 1 {
                sum += presum[curb_right_index].1 - presum[i].1
            }
            if curb_right_index <= length {
                sum += presum[length].0 - presum[curb_right_index].0;
            }
            ans = ans.max(sum);
        }
        ans
    }
}
```
