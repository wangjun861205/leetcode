Given the array restaurants where restaurants[i] = [idi, ratingi, veganFriendlyi, pricei, distancei]. You have to filter the restaurants using three filters.

The veganFriendly filter will be either true (meaning you should only include restaurants with veganFriendlyi set to true) or false (meaning you can include any restaurant). In addition, you have the filters maxPrice and maxDistance which are the maximum value for price and distance of restaurants you should consider respectively.

Return the array of restaurant IDs after filtering, ordered by rating from highest to lowest. For restaurants with the same rating, order them by id from highest to lowest. For simplicity veganFriendlyi and veganFriendly take value 1 when it is true, and 0 when it is false.

Example 1:

> Input: restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 1, maxPrice = 50, maxDistance = 10  
> Output: [3,1,5]

Explanation:  
The restaurants are:  
Restaurant 1 [id=1, rating=4, veganFriendly=1, price=40, distance=10]
Restaurant 2 [id=2, rating=8, veganFriendly=0, price=50, distance=5]
Restaurant 3 [id=3, rating=8, veganFriendly=1, price=30, distance=4]
Restaurant 4 [id=4, rating=10, veganFriendly=0, price=10, distance=3]
Restaurant 5 [id=5, rating=1, veganFriendly=1, price=15, distance=1]
After filter restaurants with veganFriendly = 1, maxPrice = 50 and maxDistance = 10 we have restaurant 3, restaurant 1 and restaurant 5 (ordered by rating from highest to lowest).

Example 2:

> Input: restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 50, maxDistance = 10  
> Output: [4,3,2,1,5]

Explanation:  
The restaurants are the same as in example 1, but in this case the filter veganFriendly = 0, therefore all restaurants are considered.

Example 3:

> Input: restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 30, maxDistance = 3  
> Output: [4,5]

Constraints:

- 1 <= restaurants.length <= 10^4
- restaurants[i].length == 5
- 1 <= idi, ratingi, pricei, distancei <= 10^5
- 1 <= maxPrice, maxDistance <= 10^5
- veganFriendlyi and veganFriendly are 0 or 1.
- All idi are distinct.

---

创建 3 个 binary heap, 其中 2 个是(price, distance, rating, id)或者(distance, price, rating, id)的 min heap, 一个是 vegan friendly 的, 另一个是全部的。 另一个是(rating, id)的 max heap。先把前两个 heap 创建出来，然后再 pop 出排好序的 restaurant，将符合条件的(rating, id)插入到第三个 heap 中。最后把第三个 heap 的 item 都 pop 出来，得到最终的答案

---

代码实现(Rust):

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Condition(i32, i32, i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rating(i32, i32);

impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        let mut vf_heap: BinaryHeap<Reverse<Condition>> = BinaryHeap::new();
        let mut all_heap: BinaryHeap<Reverse<Condition>> = BinaryHeap::new();
        for r in &restaurants {
            let cond = Reverse(Condition(r[3], r[4], r[1], r[0]));
            if r[2] == 0 {
                all_heap.push(cond);
            } else {
                all_heap.push(cond.clone());
                vf_heap.push(cond.clone());
            }
        }
        let mut h;
        if vegan_friendly == 1 {
            h = vf_heap;
        } else {
            h = all_heap;
        }
        let mut rating_heap: BinaryHeap<Rating> = BinaryHeap::new();
        while let Some(Reverse(c)) = h.pop() {
            if c.0 <= max_price {
                if c.1 <= max_distance {
                    rating_heap.push(Rating(c.2, c.3));
                }
            } else {
                break;
            }
        }
        let mut ans = Vec::new();
        while let Some(r) = rating_heap.pop() {
            ans.push(r.1);
        }
        ans
    }
}
```
