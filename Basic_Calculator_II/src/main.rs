struct Solution;

impl Solution {
    pub fn calculate(mut s: String) -> i32 {
        s = s.replace(" ", "");
        let mut l = Vec::new();
        let mut buf = String::new();
        while !s.is_empty() {
            let c = s.remove(0);
            if c == '+' || c == '-' || c == '*' || c == '/' {
                l.push(buf.clone());
                buf.clear();
                l.push(c.to_string());
            } else {
                buf.push(c);
            }
        }
        l.push(buf);
        let mut ll = Vec::new();
        for t in l {
            if !(t == "+" || t == "-" || t == "*" || t == "/") {
                if ll.len() > 1 {
                    let prev_op: String = ll.pop().unwrap();
                    let prev_num: String = ll.pop().unwrap();
                    if prev_op == "*" {
                        ll.push(
                            (prev_num.parse::<i32>().unwrap() * t.parse::<i32>().unwrap())
                                .to_string(),
                        );
                        continue;
                    }
                    if prev_op == "/" {
                        ll.push(
                            (prev_num.parse::<i32>().unwrap() / t.parse::<i32>().unwrap())
                                .to_string(),
                        );
                        continue;
                    }
                    ll.push(prev_num);
                    ll.push(prev_op);
                }
            }
            ll.push(t)
        }
        let mut ans = ll.remove(0).parse::<i32>().unwrap();
        for w in ll.chunks(2) {
            if w[0] == "+" {
                ans += w[1].parse::<i32>().unwrap();
            } else {
                ans -= w[1].parse::<i32>().unwrap();
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::calculate("1+1+1".to_owned()));
}
