struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let l_to_r: Vec<i32> = dominoes
            .chars()
            .scan(0, |s, v| {
                if v == 'R' {
                    *s += 1;
                } else if v == 'L' {
                    *s -= 1;
                } else {
                    if *s > 0 {
                        *s += 1;
                    } else if *s < 0 {
                        *s -= 1;
                    }
                }
                Some(*s)
            })
            .collect();
        let r_to_l: Vec<i32> = dominoes
            .chars()
            .rev()
            .scan(0, |s, v| {
                if v == 'L' {
                    *s += 1;
                } else if v == 'R' {
                    *s -= 1;
                } else {
                    if *s > 0 {
                        *s += 1;
                    } else if *s < 0 {
                        *s -= 1;
                    }
                }
                Some(*s)
            })
            .collect();
        (0..dominoes.len())
            .map(|i| {
                if l_to_r[i] == r_to_l[i] {
                    '.'
                } else if l_to_r[i] < r_to_l[i] {
                    'L'
                } else {
                    'R'
                }
            })
            .collect::<String>()
    }
}

fn main() {
    println!("{}", Solution::push_dominoes(".L.R...LR..L..".to_owned()));
}
