struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        if label == 1 {
            return vec![1];
        }
        let parent;
        if label % 2 == 0 {
            parent = label / 2;
        } else {
            parent = (label - 1) / 2;
        }
        let mut exp = 0;
        while 2_i32.pow(exp) > parent || 2_i32.pow(exp + 1) - 1 < parent {
            exp += 1;
        }
        let start = 2_i32.pow(exp);
        let end = 2_i32.pow(exp + 1) - 1;
        let true_parent = start + (end - parent);
        let mut path = Solution::path_in_zig_zag_tree(true_parent);
        path.push(label);
        path
    }
}

fn main() {
    println!("{:?}", Solution::path_in_zig_zag_tree(14));
}
