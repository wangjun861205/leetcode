struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; deck.len()];
        deck.sort();
        let mut idx = 0_usize;
        while !deck.is_empty() {
            let v = deck.remove(0);
            ans[idx] = v;
            if deck.is_empty() {
                break;
            }
            let mut slot_count = 0;
            loop {
                idx += 1;
                if idx == ans.len() {
                    idx = 0;
                }
                if ans[idx] == 0 {
                    if slot_count == 1 {
                        break;
                    } else {
                        slot_count = 1;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7])
    );
}
