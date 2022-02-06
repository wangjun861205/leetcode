struct Solution;

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();
        let mid = arr[(arr.len() - 1) / 2];
        let mut ans = Vec::new();
        while ans.len() < k as usize {
            if (arr[0] - mid).abs() > (*arr.last().unwrap() - mid).abs() {
                ans.push(arr.remove(0));
            } else if (arr[0] - mid).abs() < (*arr.last().unwrap() - mid).abs() {
                ans.push(arr.pop().unwrap());
            } else {
                if arr[0] > *arr.last().unwrap() {
                    ans.push(arr.remove(0));
                } else {
                    ans.push(arr.pop().unwrap());
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
