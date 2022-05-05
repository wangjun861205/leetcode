struct Solution;

impl Solution {
    fn max_presum(mut presum: Vec<i32>, length: usize) -> Vec<i32> {
        presum.insert(0, 0);
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        let mut max = 0;
        for n in presum {
            stack.push(n);
            if stack.len() < length + 1 {
                ans.push(i32::MIN);
                continue;
            }
            if stack.len() > length + 1 {
                stack.remove(0);
            }
            max = max.max(stack[stack.len() - 1] - stack[0]);
            ans.push(max);
        }
        ans
    }
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        let l = l as usize;
        let m = m as usize;
        let forward = a
            .iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect::<Vec<i32>>();
        let backward = a
            .iter()
            .rev()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect::<Vec<i32>>();
        let forward_l_max = Solution::max_presum(forward.clone(), l);
        let forward_m_max = Solution::max_presum(forward.clone(), m);
        let mut backward_l_max = Solution::max_presum(backward.clone(), l);
        let mut backward_m_max = Solution::max_presum(backward.clone(), m);
        backward_l_max.reverse();
        backward_m_max.reverse();
        let mut max = 0;
        for i in 0..a.len() + 1 {
            let fl = *forward_l_max.get(i).unwrap_or(&i32::MIN);
            let fm = *forward_m_max.get(i).unwrap_or(&i32::MIN);
            let bl = *backward_l_max.get(i).unwrap_or(&i32::MIN);
            let bm = *backward_m_max.get(i).unwrap_or(&i32::MIN);
            if fl != i32::MIN && bm != i32::MIN {
                max = max.max(fl + bm);
            }
            if fm != i32::MIN && bl != i32::MIN {
                max = max.max(fm + bl);
            }
        }
        max
    }
}
fn main() {
    println!("{}", Solution::max_sum_two_no_overlap(vec![1, 0, 3], 1, 2));
}
