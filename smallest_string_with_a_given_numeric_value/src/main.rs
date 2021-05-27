struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let l = vec![
            ('a', 1),
            ('b', 2),
            ('c', 3),
            ('d', 4),
            ('e', 5),
            ('f', 6),
            ('g', 7),
            ('h', 8),
            ('i', 9),
            ('j', 10),
            ('k', 11),
            ('l', 12),
            ('m', 13),
            ('n', 14),
            ('o', 15),
            ('p', 16),
            ('q', 17),
            ('r', 18),
            ('s', 19),
            ('t', 20),
            ('u', 21),
            ('v', 22),
            ('w', 23),
            ('x', 24),
            ('y', 25),
            ('z', 26),
        ];
        let diff = k - n;
        match diff {
            d if d < 25 => {
                let mut ans = vec!['a'; n as usize - 1];
                ans.push(l[d as usize].0);
                return ans.into_iter().collect();
            }
            _ => {
                let c = diff / 25;
                let m = diff % 25;
                if n == c {
                    return vec!['z'; n as usize].into_iter().collect();
                }
                let mut ans = vec!['a'; (n - c - 1) as usize];
                ans.push(l[m as usize].0);
                ans.append(&mut vec!['z'; c as usize]);
                return ans.into_iter().collect();
            }
        }
    }
}
fn main() {
    println!("{}", Solution::get_smallest_string(5, 130));
}
