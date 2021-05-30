struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find(parents: &mut Vec<usize>, cur: usize) -> usize {
        if parents[cur] == cur {
            return cur;
        }
        let parent = Solution::find(parents, parents[cur]);
        parents[cur] = parent;
        parent
    }

    fn union(parents: &mut Vec<usize>, x: usize, y: usize) {
        let parent_x = Solution::find(parents, x);
        let parent_y = Solution::find(parents, y);
        parents[parent_y] = parent_x;
    }

    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut parents: Vec<usize> = (0..s.len()).into_iter().collect();
        for p in pairs {
            Solution::union(&mut parents, p[0] as usize, p[1] as usize);
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut map: HashMap<usize, Vec<char>> = (0..s.len()).into_iter().fold(HashMap::new(), |mut m, i| {
            let p = Solution::find(&mut parents, i);
            m.entry(p).or_insert(Vec::new()).push(chars[i]);
            m
        });
        for (_, v) in &mut map {
            v.sort();
        }
        for i in 0..chars.len() {
            let c = map.get_mut(&parents[i]).unwrap().remove(0);
            chars[i] = c;
        }
        chars.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::smallest_string_with_swaps("dcab".to_owned(), vec![vec![0, 3], vec![1, 2], vec![0, 2]]));
}
