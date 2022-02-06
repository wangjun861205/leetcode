struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut t_ans = 0;
        let mut n = k;
        let mut stack = Vec::new();
        't: for c in answer_key.chars() {
            if c == 'F' {
                if n == 0 {
                    while stack.len() > 0 {
                        let v = stack.remove(0);
                        if v == c {
                            stack.push(c);
                            continue 't;
                        }
                    }
                } else {
                    n -= 1;
                    stack.push(c);
                    t_ans = t_ans.max(stack.len() as i32);
                }
            } else {
                stack.push(c);
                t_ans = t_ans.max(stack.len() as i32);
            }
        }
        let mut f_ans = 0;
        let mut n = k;
        let mut stack = Vec::new();
        'f: for c in answer_key.chars() {
            if c == 'T' {
                if n == 0 {
                    while stack.len() > 0 {
                        let v = stack.remove(0);
                        if v == c {
                            stack.push(c);
                            continue 'f;
                        }
                    }
                } else {
                    n -= 1;
                    stack.push(c);
                    f_ans = f_ans.max(stack.len() as i32);
                }
            } else {
                stack.push(c);
                f_ans = f_ans.max(stack.len() as i32);
            }
        }
        t_ans.max(f_ans)
    }
}

fn main() {
    println!("{}", Solution::max_consecutive_answers("TTFF".into(), 2));
}
