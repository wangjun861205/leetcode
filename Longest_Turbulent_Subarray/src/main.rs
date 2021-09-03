struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut list: Vec<char> = arr
            .windows(2)
            .map(|w| {
                if w[0] < w[1] {
                    '<'
                } else if w[0] > w[1] {
                    '>'
                } else {
                    '='
                }
            })
            .collect();
        if list.len() == 0 {
            return 1;
        }
        let mut ans = 0;
        let mut count = 0;
        let mut prev = '=';
        while !list.is_empty() {
            let c = list.remove(0);
            match c {
                '=' => {
                    ans = ans.max(count);
                    prev = '=';
                    count = 0;
                }
                '>' => {
                    if prev != '>' {
                        count += 1;
                        prev = '>';
                    } else {
                        ans = ans.max(count);
                        count = 1;
                    }
                }
                _ => {
                    if prev != '<' {
                        count += 1;
                        prev = '<';
                    } else {
                        ans = ans.max(count);
                        count = 1;
                    }
                }
            }
        }
        ans = ans.max(count);
        ans + 1
    }
}

fn main() {
    println!("{}", Solution::max_turbulence_size(vec![0, 8, 45, 88, 48, 68, 28, 55, 17, 24]));
}
