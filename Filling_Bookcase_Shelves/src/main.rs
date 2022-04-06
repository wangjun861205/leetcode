struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        books: &Vec<Vec<i32>>,
        shelf_width: i32,
        index: usize,
        remain_width: i32,
        max_height: i32,
        cache: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if index == books.len() {
            return max_height;
        }
        let thickness = books[index][0];
        let height = books[index][1];
        if remain_width < thickness {
            let next = if let Some(c) = cache.get(&(index + 1, shelf_width - thickness, height)) {
                *c
            } else {
                Solution::dp(
                    books,
                    shelf_width,
                    index + 1,
                    shelf_width - thickness,
                    height,
                    cache,
                )
            };
            cache.insert((index, remain_width, max_height), max_height + next);
            return max_height + next;
        }

        let a = if let Some(c) = cache.get(&(index + 1, shelf_width - thickness, height)) {
            *c
        } else {
            Solution::dp(
                books,
                shelf_width,
                index + 1,
                shelf_width - thickness,
                height,
                cache,
            )
        } + max_height;
        let b = if let Some(c) =
            cache.get(&(index + 1, remain_width - thickness, max_height.max(height)))
        {
            *c
        } else {
            Solution::dp(
                books,
                shelf_width,
                index + 1,
                remain_width - thickness,
                max_height.max(height),
                cache,
            )
        };
        let ans = a.min(b);
        cache.insert((index, remain_width, max_height), ans);
        ans
    }
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        Solution::dp(&books, shelf_width, 0, shelf_width, 0, &mut HashMap::new())
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6)
    );
}
