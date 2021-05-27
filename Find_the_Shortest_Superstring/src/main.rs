struct Solution;

impl Solution {
    fn merge(s: Vec<char>, w: Vec<char>) -> Vec<char> {
        let mut len = s.len().min(w.len());
        while len > 0 {
            let left = s[s.len() - len..].to_vec();
            let right = w[..len].to_vec();
            if left == right {
                break;
            } else {
                len -= 1;
            }
        }
        let mut ss = s.clone();
        ss.append(&mut w[len..].to_vec());
        ss
    }
    fn overlap(s: &Vec<char>, w: &Vec<char>) -> i32 {
        let mut len = s.len().min(w.len()) as i32;
        while len > 0 {
            let left = s[s.len() - len as usize..].to_vec();
            let right = w[..len as usize].to_vec();
            if left == right {
                break;
            } else {
                len -= 1;
            }
        }
        len
    }
    fn rc(words_count: usize, mask: usize, index: usize, overlaps: &Vec<Vec<i32>>, cache: &mut Vec<Vec<(i32, Vec<usize>)>>) -> (i32, Vec<usize>) {}

    pub fn shortest_superstring(words: Vec<String>) -> String {
        let words: Vec<Vec<char>> = words.into_iter().map(|w| w.chars().collect()).collect();
        let mut overlaps: Vec<Vec<i32>> = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i != j {
                    overlaps[i][j] = Solution::overlap(&words[i], &words[j]);
                }
            }
        }
        let mut dp: Vec<Vec<(i32, Vec<usize>)>> = vec![vec![(-1, vec![]); words.len()]; 1 << words.len()];
        for i in 0..words.len() {
            dp[1 << i][i] = (0, vec![i]);
        }
        let (_, order) = Solution::rc(words.len(), (1 << words.len()) - 1, &overlaps, &mut dp);
        let mut ans: Vec<char> = Vec::new();
        for i in order {
            ans = Solution::merge(ans, words[i].clone());
        }
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn merge_test() {
        use super::Solution;
        println!("{:?}", Solution::overlap(&vec!['a', 'b', 'c', 'd'], &vec!['e', 'd', 'b']));
    }
}
fn main() {
    println!(
        "{}",
        Solution::shortest_superstring(vec!["catg".to_owned(), "ctaagt".to_owned(), "gcta".to_owned(), "ttca".to_owned(), "atgcatc".to_owned()])
    );
}
