struct Solution;

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut length_groups = vec![Vec::new(); 100];
        for n in nums {
            length_groups[100 - n.len()].push(n);
        }
        let mut count = 0;
        for mut group in length_groups {
            if count + group.len() as i32 >= k {
                group.sort();
                group.reverse();
                return group[(k - count - 1) as usize].clone();
            }
            count += group.len() as i32;
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
