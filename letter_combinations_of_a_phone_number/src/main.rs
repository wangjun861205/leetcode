use std::collections::HashMap;
use std::str;

struct Solution;

impl Solution {
    fn rc(
        digits: String,
        map: &HashMap<char, Vec<char>>,
        cache: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut ans = Vec::<String>::new();
        if digits.len() == 0 {
            return ans;
        }
        if digits.len() == 1 {
            let b = digits.as_bytes().first().unwrap();
            return map
                .get(&(*b as char))
                .unwrap()
                .iter()
                .map(|c| c.to_string())
                .collect();
        }
        let d = digits.as_bytes()[0] as char;
        let next = str::from_utf8(&digits.as_bytes()[1..]).unwrap().to_owned();
        if let Some(l) = cache.get(&next) {
            for c in map.get(&d).unwrap() {
                for n in l.iter() {
                    let mut s = c.to_string();
                    s.push_str(n);
                    ans.push(s);
                }
            }
        } else {
            let l = Solution::rc(next, map, cache);
            for c in map.get(&d).unwrap() {
                for n in l.iter() {
                    let mut s = c.to_string();
                    s.push_str(n);
                    ans.push(s);
                }
            }
        }
        cache.insert(digits, ans.clone());
        ans
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: HashMap<char, Vec<char>> = vec![
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]
        .into_iter()
        .collect();
        Solution::rc(digits, &map, &mut HashMap::new())
    }
}
fn main() {
    println!("{:?}", Solution::letter_combinations("234".to_owned()));
}
