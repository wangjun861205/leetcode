struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        // 依据s生成一个二位数组， 记录每一个字母出现的index
        let mut c_to_i: Vec<Vec<i32>> = vec![vec![]; 26];
        for (i, c) in s.chars().enumerate() {
            c_to_i[c as usize - 97].push(i as i32);
        }
        let mut ans = String::new();
        'outer: for word in dictionary {
            let mut prev_pos = -1;
            for c in word.chars() {
                // 如果字母没有在s中出现则直接检查下一个word
                if c_to_i[c as usize - 97].is_empty() {
                    continue 'outer;
                } else {
                    // 找到第一个大于前一个字符index的位置
                    if let Some(&pos) = c_to_i[c as usize - 97].iter().find(|&v| v > &prev_pos) {
                        // 如果存在则把前一个字符index更新为当前的index
                        prev_pos = pos;
                    } else {
                        // 如果不存在则直接下一个单词
                        continue 'outer;
                    }
                }
            }
            if word.len() > ans.len() || (word.len() == ans.len() && word < ans) {
                ans = word;
            }
        }
        ans
    }
}

fn main() {
    let s = "aewfafwafjlwajflwajflwafj".to_owned();
    let dict = vec![
        "apple".to_owned(),
        "ewaf".to_owned(),
        "awefawfwaf".to_owned(),
        "awef".to_owned(),
        "awefe".to_owned(),
        "ewafeffewafewf".to_owned(),
    ];
    println!("{}", Solution::find_longest_word(s, dict))
}
