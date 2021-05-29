struct Solution;

impl Solution {
    fn find(parents: &mut Vec<i32>, cur: i32) -> i32 {
        if parents[cur as usize] == cur {
            return cur;
        } else {
            let parent = Solution::find(parents, parents[cur as usize]);
            parents[cur as usize] = parent;
            return parent;
        }
    }

    fn union(parents: &mut Vec<i32>, ranks: &mut Vec<i32>, x: i32, y: i32) -> bool {
        let parent_x = Solution::find(parents, x);
        let parent_y = Solution::find(parents, y);
        if parent_x == parent_y {
            return false;
        }
        if ranks[parent_x as usize] > ranks[parent_y as usize] {
            parents[parent_y as usize] = parent_x;
        } else if ranks[parent_x as usize] < ranks[parent_y as usize] {
            parents[parent_x as usize] = parent_y;
        } else {
            ranks[parent_x as usize] += 1;
            parents[parent_y as usize] = parent_x;
        }
        return true;
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parents: Vec<i32> = (0..1001).into_iter().collect();
        let mut ranks: Vec<i32> = vec![0; 1001];
        for e in edges {
            if !Solution::union(&mut parents, &mut ranks, e[0], e[1]) {
                return e;
            }
        }
        unreachable!();
    }
}
fn main() {
    println!("Hello, world!");
}
