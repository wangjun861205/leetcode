struct Solution;

impl Solution {
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut time = 1;
        for i in 1..i32::MAX {
            if memory1 < memory2 {
                if memory2 >= i {
                    memory2 -= i;
                    time += 1;
                    continue;
                }
            } else {
                if memory1 >= i {
                    memory1 -= i;
                    time += 1;
                    continue;
                }
            }
            break;
        }
        vec![time, memory1, memory2]
    }
}
fn main() {
    println!("{:?}", Solution::mem_leak(2, 2));
}
