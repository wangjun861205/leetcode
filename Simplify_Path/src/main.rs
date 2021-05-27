struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<String> = Vec::new();
        let mut buffer = String::new();
        for c in path.chars().skip(1) {
            if c == '/' {
                if buffer.len() == 0 {
                    continue;
                }
                if buffer == "." {
                    buffer.clear();
                    continue;
                }
                if buffer == ".." {
                    buffer.clear();
                    if stack.len() > 0 {
                        stack.pop();
                    }
                    continue;
                }
                stack.push(buffer.clone());
                buffer.clear();
            } else {
                buffer.push(c);
            }
        }
        if buffer.len() > 0 {
            match buffer.as_str() {
                "." => {}
                ".." => {
                    if stack.len() > 0 {
                        stack.pop();
                    }
                }
                _ => stack.push(buffer),
            }
        }
        let mut ans = stack.join("/");
        ans.insert(0, '/');
        ans
    }
}
fn main() {
    println!("{}", Solution::simplify_path("/a/./b/../../c/".to_owned()));
}
