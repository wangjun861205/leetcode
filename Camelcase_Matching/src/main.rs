struct Solution;

impl Solution {
    fn cmp(query: &String, pattern: &String) -> bool {
        let q: Vec<char> = query.chars().collect();
        let p: Vec<char> = pattern.chars().collect();
        let mut qi = 0_usize;
        let mut pi = 0_usize;
        'outer: while pi < p.len() {
            while qi < q.len() {
                if q[qi] != p[pi] {
                    if p[pi].is_uppercase() && q[qi].is_uppercase() {
                        return false;
                    } else if p[pi].is_uppercase() && q[qi].is_lowercase() {
                        qi += 1;
                        continue;
                    } else if p[pi].is_lowercase() && q[qi].is_uppercase() {
                        return false;
                    } else {
                        qi += 1;
                        continue;
                    }
                }
                pi += 1;
                qi += 1;
                continue 'outer;
            }
            return false;
        }
        while qi < q.len() {
            if q[qi].is_uppercase() {
                return false;
            }
            qi += 1;
        }
        true
    }
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries.iter().map(|v| Solution::cmp(v, &pattern)).collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::camel_match(
            vec![
                "FooBar",
                "FooBarTest",
                "FootBall",
                "FrameBuffer",
                "ForceFeedBack"
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            "FB".to_owned()
        )
    );
}
