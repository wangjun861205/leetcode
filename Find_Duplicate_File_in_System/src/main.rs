struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<String, Vec<String>> = HashMap::new();
        for p in paths {
            let mut l: Vec<String> = p.split(" ").map(|v| v.to_owned()).collect();
            let path = l.remove(0);
            for mut f in l {
                let mut filename = String::new();
                loop {
                    let c = f.remove(0);
                    if c != '(' {
                        filename.push(c);
                        continue;
                    }
                    break;
                }
                let mut content = f;
                content.pop();
                m.entry(content)
                    .or_insert(Vec::new())
                    .push(vec![path.clone(), filename].join("/"));
            }
        }
        m.values()
            .filter(|v| v.len() > 1)
            .map(|v| v.clone())
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
