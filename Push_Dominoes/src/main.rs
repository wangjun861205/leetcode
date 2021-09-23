struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let dominoes: Vec<char> = dominoes.chars().into_iter().collect();
        let mut result = vec![0; dominoes.len()];
        let mut second = 1;
        let mut started = false;
        for i in 0..dominoes.len() {
            match dominoes[i] {
                '.' => {
                    if started {
                        result[i] = second;
                    }
                }
                'R' => {
                    second = 1;
                    started = true;
                    result[i] = second;
                }
                _ => {
                    started = false;
                }
            }
            second += 1;
        }
        second = -1;
        started = false;
        for i in (0..dominoes.len()).rev() {
            match dominoes[i] {
                '.' => {
                    if started {
                        if result[i] == 0 {
                            result[i] = second;
                        } else {
                            if result[i] + second > 0 {
                                result[i] = second;
                            } else if result[i] + second == 0 {
                                result[i] = 0;
                                started = false;
                            }
                        }
                    }
                }
                'L' => {
                    second = -1;
                    started = true;
                    result[i] = second;
                }
                _ => {
                    started = false;
                }
            }
            second -= 1;
        }

        result
            .into_iter()
            .map(|v| {
                if v > 0 {
                    'R'
                } else if v < 0 {
                    'L'
                } else {
                    '.'
                }
            })
            .collect()
    }
}

fn main() {
    println!("{}", Solution::push_dominoes(".L.R...LR..L..".to_owned()));
}
