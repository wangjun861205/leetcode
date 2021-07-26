struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn check_word(w1: &String, w2: &String) -> bool {
        let mut count = 0;
        for (c1, c2) in w1.chars().zip(w2.chars()) {
            if c1 != c2 {
                count += 1;
            }
            if count == 2 {
                return false;
            }
        }
        return count == 1;
    }

    fn bfs(result: Vec<Vec<String>>, visited: HashSet<String>, end_word: &String, word_list: &Vec<String>) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut vis = HashSet::new();
        for l in result.clone() {
            let last = l.last().unwrap();
            if last == end_word {
                res.push(l.clone());
                continue;
            }
            for w in word_list {
                if Solution::check_word(last, w) && !visited.contains(w) {
                    let mut ll = l.clone();
                    ll.push(w.clone());
                    res.push(ll);
                    vis.insert(w.clone());
                }
            }
        }
        if vis.len() == 0 {
            return result;
        }
        let v: HashSet<String> = visited.union(&vis).map(|s| s.clone()).collect();
        Solution::bfs(res, v, end_word, word_list)
    }

    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let result = vec![vec![begin_word.clone()]];
        let visited = HashSet::from_iter(vec![begin_word.clone()].into_iter());
        let res = Solution::bfs(result, visited, &end_word, &word_list);
        res.into_iter().filter(|l| l.last().unwrap() == &end_word).collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_ladders(
            "lost".to_owned(),
            "miss".to_owned(),
            vec!["most", "mist", "miss", "lost", "fist", "fish"].into_iter().map(|v| v.to_owned()).collect()
        )
    );
}
