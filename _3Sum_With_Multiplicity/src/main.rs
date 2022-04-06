struct Solution;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let counts = arr.iter().fold(vec![0i128; 101], |mut l, v| {
            l[*v as usize] += 1;
            l
        });
        let mut ans = 0i128;
        const MOD: i128 = 10i128.pow(9) + 7;
        for x in 0..=100 {
            if counts[x as usize] == 0 {
                continue;
            }
            for y in x + 1..=100 {
                if counts[y as usize] == 0 {
                    continue;
                }
                let z = target - x - y;
                if z <= y || z > 100 {
                    continue;
                }
                ans += counts[x as usize] * counts[y as usize] * counts[z as usize] % MOD;
                ans %= MOD;
            }
        }
        for x in 0..=100 {
            for y in x + 1..=100 {
                if counts[y as usize] >= 2 && x + y * 2 == target {
                    ans += counts[x as usize] * counts[y as usize] * (counts[y as usize] - 1) / 2 % MOD;
                    ans %= MOD;
                }
            }
        }
        for x in 0..=100 {
            for y in x + 1..=100 {
                if counts[x as usize] >= 2 && 2 * x + y == target {
                    ans += counts[x as usize] * (counts[x as usize] - 1) * counts[y as usize] / 2 % MOD;
                    ans %= MOD;
                }
            }
        }
        if target % 3 == 0 {
            let x = target / 3;
            if counts[x as usize] >= 3 {
                ans += counts[x as usize] * (counts[x as usize] - 1) * (counts[x as usize] - 2) / 6 % MOD;
                ans %= MOD;
            }
        }
        ans as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::three_sum_multi(
            vec![
                92, 4, 59, 23, 100, 16, 7, 15, 3, 78, 98, 17, 77, 33, 83, 15, 87, 35, 54, 72, 58, 14, 87, 47, 58, 31, 72, 58, 87, 22, 25, 54, 27, 53, 13, 54, 61, 12, 96, 24, 35, 43, 94, 1, 88, 76,
                89, 89, 41, 56, 61, 65, 60, 91, 89, 79, 86, 52, 27, 2, 97, 46, 50, 46, 87, 93, 71, 87, 95, 78, 65, 10, 35, 51, 34, 66, 61, 7, 49, 38, 10, 1, 88, 37, 50, 84, 35, 20, 7, 83, 51, 85, 11,
                12, 89, 93, 54, 23, 36, 95, 100, 19, 82, 67, 96, 77, 53, 56, 51, 16, 54, 7, 30, 68, 78, 13, 38, 52, 91, 44, 54, 17, 21, 44, 4, 10, 85, 19, 11, 88, 73, 36, 47, 53, 3, 21, 41, 24, 60,
                53, 88, 35, 48, 89, 35, 3, 43, 85, 45, 67, 56, 78, 44, 49, 29, 35, 68, 96, 29, 21, 51, 17, 52, 99, 3, 48, 65, 51, 22, 38, 77, 81, 30, 64, 99, 35, 88, 72, 73, 29, 29, 2
            ],
            105
        )
    );
}
