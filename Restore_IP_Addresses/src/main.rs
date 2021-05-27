struct Solution;

impl Solution {
    fn check(seg: &Vec<char>) -> bool {
        match seg.len() {
            1 => {
                return true;
            }
            2 => {
                if seg[0] == '0' {
                    return false;
                }
                return true;
            }
            3 => match seg[0] {
                '1' => return true,
                '2' => match seg[1] {
                    '0' | '1' | '2' | '3' | '4' => {
                        return true;
                    }
                    '5' => match seg[2] {
                        '0' | '1' | '2' | '3' | '4' | '5' => return true,
                        _ => return false,
                    },
                    _ => return false,
                },
                _ => return false,
            },
            _ => return false,
        }
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 4 || chars.len() > 12 {
            return vec![];
        }
        if !chars.iter().all(|c| c.is_numeric()) {
            return vec![];
        }
        let mut ans: Vec<String> = Vec::new();

        'outer: for i in 0..3 {
            if i == chars.len() {
                break;
            }
            let seg1 = chars[..=i].to_vec();
            'middle: for j in i + 1..i + 4 {
                if j == chars.len() {
                    continue 'outer;
                }
                let seg2 = chars[i + 1..=j].to_vec();
                'inner: for k in j + 1..j + 4 {
                    if k == chars.len() {
                        continue 'middle;
                    }
                    if k < chars.len() - 4 {
                        continue;
                    }
                    let seg3 = chars[j + 1..=k].to_vec();
                    let seg4 = chars[k + 1..].to_vec();
                    let l = vec![seg1.clone(), seg2.clone(), seg3.clone(), seg4.clone()];
                    if !l.iter().all(Solution::check) {
                        continue;
                    }
                    let addr = l
                        .into_iter()
                        .map(|v| v.into_iter().collect::<String>())
                        .collect::<Vec<String>>()
                        .join(".");
                    ans.push(addr)
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::restore_ip_addresses("172162541".to_owned())
    );
}
