struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if let Some(pos) = arr.iter().position(|v| v == &x) {
            match pos {
                0 => return arr[..k as usize].to_vec(),
                p if p == arr.len() - 1 => return arr[arr.len() - k as usize..].to_vec(),
                _ => {
                    let mut left_index = pos;
                    let mut right_index = pos;
                    while right_index - left_index + 1 < k as usize {
                        if left_index != 0 {
                            if right_index != arr.len() - 1 {
                                if x - arr[left_index - 1] <= arr[right_index + 1] - x {
                                    left_index -= 1;
                                } else {
                                    right_index += 1;
                                }
                            } else {
                                left_index -= 1;
                            }
                        } else {
                            right_index += 1;
                        }
                    }
                    arr[left_index..=right_index].to_vec()
                }
            }
        } else {
            let mut pos = 0_usize;
            if x > arr[arr.len() - 1] {
                pos = arr.len() - 1;
            } else {
                for (i, w) in arr.windows(2).enumerate() {
                    if w[1] > x && w[0] < x {
                        if x - w[0] <= w[1] - x {
                            pos = i;
                            break;
                        } else {
                            pos = i + 1;
                            break;
                        }
                    }
                }
            }
            let mut left_index = pos;
            let mut right_index = pos;
            while right_index - left_index + 1 < k as usize {
                if left_index != 0 {
                    if right_index != arr.len() - 1 {
                        if x - arr[left_index - 1] <= arr[right_index + 1] - x {
                            left_index -= 1;
                        } else {
                            right_index += 1;
                        }
                    } else {
                        left_index -= 1;
                    }
                } else {
                    right_index += 1;
                }
            }
            arr[left_index..=right_index].to_vec()
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1)
    );
}
