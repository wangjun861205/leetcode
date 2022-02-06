> Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 109 + 7.

Example 1:

> Input: arr = [3,1,2,4]
> Output: 17
> Explanation:
> Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
> Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
> Sum is 17.

Example 2:

> Input: arr = [11,81,94,43,3]
> Output: 444

Constraints:

> 1 <= arr.length <= 3 _ 104
> 1 <= arr[i] <= 3 _ 104

---

这题的核心其实是要换个角度看问题，我们不能从 subarray 的角度来找每个 subarray 里面最小的那个 number， 而是应该从 number 的角度来找，有多少个 subarray 可以包含当前 number 并且当前 number 还是其中最小的元素。
要想找出符合条件的 subarray 的数量， 我们仅需要查找当前元素 arr[i]的左侧和右侧连续的大于 arr[i]的元素数量.

例如: [3, 1, 2, 4], i = 2  
arr[i] = 2  
向左查找: arr[i-1] = 1, 1 < 2, 不符合条件, 所以左侧的数量是 1(包含 arr[i])  
向右查找: arr[i+1] = 4, 4 > 2, 符合条件， 所有右侧的数量是 2(包含 arr[i])  
所以对于 arr[2]这个元素，实际包含该元素并且该元素为最小值的 subarray 的数量应该是 1 \* 2 = 2.

但是这里需要注意一点的是 arr 中可能会出现重复元素， 而重复元素又可能会导致重复计数。  
例如: [71, 55, 84, 55]
当 i = 1 和 i = 3 的时候我们都会把[55, 84, 55]计进来， 这样就会造成重复
解决这一问题的办法就是向右查找的时候我们会把相等的元素计进来， 但是向左的时候我们不计入相等的元素

---

```rust
impl Solution {
    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        let l: Vec<i128> = arr.iter().map(|&v| v as i128).collect();
        let search_left = |i: usize| {
            let mut count = 0_i128;
            for v in arr[..i].iter().rev() {
                if v <= &arr[i] {
                    break;
                }
                count += 1;
            }
            return count + 1;
        };

        let search_right = |i: usize| {
            let mut count = 0;
            for v in arr[i + 1..].iter() {
                if v < &arr[i] {
                    break;
                }
                count += 1;
            }
            return count + 1;
        };
        let mut ans: i128 = 0;
        for i in 0..arr.len() {
            let left = search_left(i);
            let right = search_right(i);
            ans += left * right * l[i];
        }
        (ans % 1000000007) as i32
    }
}
```
