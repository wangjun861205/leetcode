struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find(parents: &mut HashMap<String, String>, val: &String) -> String {
        let p = parents.get(val).unwrap().clone();
        if &p == val {
            return p;
        } else {
            let pp = Solution::find(parents, &p);
            *parents.get_mut(&p).unwrap() = pp.clone();
            return pp;
        }
    }
    fn union(parents: &mut HashMap<String, String>, v1: &String, v2: &String) {
        let p1 = Solution::find(parents, v1);
        let p2 = Solution::find(parents, v2);
        *parents.get_mut(&p2).unwrap() = p1;
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let names: HashMap<String, String> = accounts
            .iter()
            .flat_map(|v| {
                let name = v[0].clone();
                let vv: Vec<(String, String)> = v
                    .into_iter()
                    .skip(1)
                    .map(|e| (e.clone(), name.clone()))
                    .collect();
                vv.into_iter()
            })
            .collect();
        let mut parents: HashMap<String, String> =
            names.keys().map(|k| (k.clone(), k.clone())).collect();
        for v in accounts {
            for w in v[1..].windows(2) {
                Solution::union(&mut parents, &w[0], &w[1]);
            }
        }
        let mut ans: HashMap<String, Vec<String>> = HashMap::new();
        let emails: Vec<String> = parents.keys().map(|k| k.clone()).collect();
        for e in emails {
            let p = Solution::find(&mut parents, &e);
            ans.entry(p).or_insert(Vec::new()).push(e);
        }
        ans.into_iter()
            .map(|(k, mut v)| {
                let name = names.get(&k).unwrap().clone();
                v.sort();
                v.insert(0, name);
                v
            })
            .collect::<Vec<Vec<String>>>()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::accounts_merge(vec![
            vec![
                "David".to_owned(),
                "David0@m.co".to_owned(),
                "David1@m.co".to_owned()
            ],
            vec![
                "David".to_owned(),
                "David3@m.co".to_owned(),
                "David4@m.co".to_owned()
            ],
            vec![
                "David".to_owned(),
                "David4@m.co".to_owned(),
                "David5@m.co".to_owned()
            ],
            vec![
                "David".to_owned(),
                "David2@m.co".to_owned(),
                "David3@m.co".to_owned()
            ],
            vec![
                "David".to_owned(),
                "David1@m.co".to_owned(),
                "David2@m.co".to_owned()
            ]
        ])
    );
}
