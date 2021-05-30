struct Solution;

impl Solution {
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        let mut chars: Vec<char> = Vec::new();
        while !(a + b == 0 || b + c == 0 || a + c == 0) {
            if a != 0 {
                chars.push('a');
                a -= 1;
            }
            if b != 0 {
                chars.push('b');
                b -= 1;
            }
            if c != 0 {
                chars.push('c');
                c -= 1;
            }
        }
        if a + b + c == 0 {
            return chars.into_iter().collect();
        }
        if chars.len() == 1 {
            return chars.into_iter().collect();
        }
        let (left_char, mut left_count) = if a + b == 0 {
            ('c', c)
        } else if a + c == 0 {
            ('b', b)
        } else {
            ('a', a)
        };

        let mut idx = 0;
        while idx < chars.len() - 1 && left_count > 0 {
            if chars[idx] == left_char && chars[idx + 1] != left_char {
                chars.insert(idx + 1, left_char);
                left_count -= 1;
                idx += 2;
            } else if chars[idx] != left_char && chars[idx + 1] == left_char {
                chars.insert(idx + 1, left_char);
                left_count -= 1;
                idx += 3;
            } else if chars[idx] != left_char && chars[idx + 1] != left_char {
                chars.insert(idx + 1, left_char);
                left_count -= 1;
                idx += 1;
            }
        }
        if left_count == 0 {
            return chars.into_iter().collect();
        }
        if chars[0] != left_char {
            for _ in 0..2.min(left_count) {
                chars.insert(0, left_char);
                left_count -= 1;
            }
        }
        if left_count == 0 {
            return chars.into_iter().collect();
        }
        if chars[chars.len() - 1] != left_char {
            for _ in 0..2.min(left_count) {
                chars.push(left_char);
                left_count -= 1;
            }
        }
        chars.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::longest_diverse_string(2, 4, 1));
}
