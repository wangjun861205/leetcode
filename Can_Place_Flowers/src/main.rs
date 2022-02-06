struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() as i32 {
            if flowerbed[i as usize] == 0 {
                if i - 1 >= 0 {
                    if i + 1 < flowerbed.len() as i32 {
                        if flowerbed[(i - 1) as usize] == 0 && flowerbed[(i + 1) as usize] == 0 {
                            flowerbed[i as usize] = 1;
                            n -= 1;
                        }
                    } else {
                        if flowerbed[(i - 1) as usize] == 0 {
                            flowerbed[i as usize] = 1;
                            n -= 1;
                        }
                    }
                } else {
                    if i + 1 < flowerbed.len() as i32 {
                        if flowerbed[(i + 1) as usize] == 0 {
                            flowerbed[i as usize] = 1;
                            n -= 1;
                        }
                    } else {
                        flowerbed[i as usize] = 1;
                        n -= 1;
                    }
                }
            }
        }
        n <= 0
    }
}

fn main() {
    println!("Hello, world!");
}
