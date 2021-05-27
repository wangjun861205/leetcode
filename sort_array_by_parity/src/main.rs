struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut even_index = 0_usize;
        let mut odd_index = a.len() - 1;
        while even_index < odd_index {
            if a[even_index] % 2 == 1 {
                if a[odd_index] % 2 == 0 {
                    a.swap(even_index, odd_index);
                } else {
                    odd_index -= 1;
                }
            } else {
                even_index += 1;
            }
        }
        a
    }
}
fn main() {
    println!("{:?}", Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
}
