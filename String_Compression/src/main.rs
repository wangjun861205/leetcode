struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut read_index = 0usize;
        let mut write_index = 0usize;
        let mut prev_char = None;
        let mut count = 0;
        while read_index < chars.len() {
            let curr = chars[read_index];
            if let Some(prev) = prev_char {
                if curr == prev {
                    count += 1;
                    read_index += 1;
                    continue;
                }
                if count > 1 {
                    for c in format!("{}{}", prev, count).chars() {
                        chars[write_index] = c;
                        write_index += 1;
                    }
                } else {
                    chars[write_index] = prev;
                    write_index += 1;
                }
                read_index += 1;
                count = 1;
                prev_char = Some(curr);
                continue;
            }
            prev_char = Some(curr);
            read_index = 1;
            count = 1;
        }
        if count > 1 {
            for c in format!("{}{}", prev_char.as_ref().unwrap(), count).chars() {
                chars[write_index] = c;
                write_index += 1;
            }
        } else {
            chars[write_index] = *prev_char.as_ref().unwrap();
            write_index += count;
        }
        chars.truncate(write_index);
        chars.len() as i32
    }
}

fn main() {
    let mut l = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    println!("{}", Solution::compress(&mut l));
    println!("{:?}", l);
}
