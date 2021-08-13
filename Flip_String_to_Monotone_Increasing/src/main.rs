struct Solution;

impl Solution {
    fn dp(chars: &Vec<char>, i: usize, zero_count: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        let mut first_one_index = None;
        for j in i..chars.len() {
            if chars[j] == '1' {
                first_one_index = Some(j);
                break;
            }
        }
        if let Some(idx) = first_one_index {
            if idx == chars.len() - 1 {
                return 0;
            }
            let non_flap = zero_count[idx];
            let flap = if cache[idx + 1] != -1 {
                cache[idx + 1]
            } else {
                Solution::dp(chars, idx + 1, zero_count, cache)
            };
            let ans = non_flap.min(flap + 1);
            cache[i] = ans;
            return ans;
        }
        0
    }

    pub fn min_flips_mono_incr(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut zero_count: Vec<i32> = chars
            .iter()
            .rev()
            .scan(0, |c, v| {
                if v == &'0' {
                    *c += 1;
                }
                Some(*c)
            })
            .collect();
        zero_count.reverse();
        let mut cache = vec![-1; chars.len()];
        Solution::dp(&chars, 0, &zero_count, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_flips_mono_incr("011010001101001".into())
    );
}
