struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        if k == 1 {
            return nums;
        }
        if len as i32 == k {
            return vec![nums.into_iter().max().unwrap()];
        }
        let mut queue: Vec<i32> = Vec::new();
        let mut indices: Vec<i32> = Vec::new();
        for (i, v) in nums[..k as usize].iter().enumerate() {
            let (mut new_queue, mut new_indices): (Vec<i32>, Vec<i32>) = queue
                .clone()
                .into_iter()
                .zip(indices.clone().into_iter())
                .filter(|(p, _)| p > v)
                .unzip();
            new_queue.push(*v);
            new_indices.push(i as i32);
            queue = new_queue;
            indices = new_indices;
        }
        let mut ans: Vec<i32> = vec![queue[0]];
        for (i, v) in nums.iter().enumerate().skip(k as usize) {
            if v > &queue[0] {
                queue = vec![*v];
                indices = vec![i as i32];
                ans.push(*v);
                continue;
            }
            let (mut new_queue, mut new_indices): (Vec<i32>, Vec<i32>) = queue
                .clone()
                .into_iter()
                .zip(indices.clone().into_iter())
                .filter(|(p, _)| p > v)
                .unzip();
            new_queue.push(*v);
            new_indices.push(i as i32);
            if new_indices[0] + k <= i as i32 {
                new_queue.remove(0);
                new_indices.remove(0);
            }
            queue = new_queue;
            indices = new_indices;
            ans.push(queue[0]);
        }
        ans
    }
}

fn main() {}

// [9, 10, 9, -7, -4]

// [10]
// [1]
// [10, -8]
// [1, 5]
// []
