struct Solution;

impl Solution {
    fn find_l(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in i..chars.len() {
            match chars[j] {
                'L' => {
                    return Some(j);
                }
                'X' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_r(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in (0..=i).rev() {
            match chars[j] {
                'R' => return Some(j),
                'X' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_rx(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in i..chars.len() {
            match chars[j] {
                'X' => return Some(j),
                'R' => continue,
                _ => return None,
            }
        }
        None
    }

    fn find_lx(chars: &Vec<char>, i: usize) -> Option<usize> {
        for j in (0..=i).rev() {
            match chars[j] {
                'X' => return Some(j),
                'L' => continue,
                _ => return None,
            }
        }
        None
    }

    pub fn can_transform(start: String, end: String) -> bool {
        let mut start_chars: Vec<char> = start.chars().collect();
        let end_chars: Vec<char> = end.chars().collect();
        for i in 0..start_chars.len() {
            let s = start_chars[i];
            let e = end_chars[i];
            if s == e {
                continue;
            }
            match e {
                'X' => match s {
                    'R' => {
                        if let Some(j) = Solution::find_rx(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        if let Some(j) = Solution::find_lx(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                },
                'R' => match s {
                    'X' => {
                        if let Some(j) = Solution::find_r(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                },
                _ => match s {
                    'X' => {
                        if let Some(j) = Solution::find_l(&start_chars, i) {
                            let temp = start_chars[i];
                            start_chars[i] = start_chars[j];
                            start_chars[j] = temp;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                },
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_transform("RXXLRXRXL".into(), "XRLXXRRLX".into())
    );
    println!("{}", Solution::can_transform("X".into(), "L".into()));
    println!("{}", Solution::can_transform("LLR".into(), "RRL".into()));
    println!("{}", Solution::can_transform("XLLR".into(), "LXLX".into()));
}
