struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        if first_list.len() == 0 || second_list.len() == 0 {
            return Vec::new();
        }
        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut ans: Vec<Vec<i32>> = Vec::new();
        while idx1 < first_list.len() && idx2 < second_list.len() {
            let range1 = &first_list[idx1];
            let range2 = &second_list[idx2];
            let (s1, e1) = (range1[0], range1[1]);
            let (s2, e2) = (range2[0], range2[1]);
            if e1 < s2 {
                idx1 += 1;
                continue;
            }
            if e2 < s1 {
                idx2 += 1;
                continue;
            }
            if s1 <= s2 && e1 <= e2 {
                ans.push(vec![s2, e1]);
                idx1 += 1;
            } else if s1 >= s2 && e1 >= e2 {
                ans.push(vec![s1, e2]);
                idx2 += 1;
            } else if s1 <= s2 && e1 >= e2 {
                ans.push(vec![s2, e2]);
                idx2 += 1;
            } else if s1 >= s2 && e1 <= e2 {
                ans.push(vec![s1, e1]);
                idx1 += 1;
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        )
    );
}
