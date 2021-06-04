struct Solution;

impl Solution {
    fn dfs(prereq: &Vec<Vec<usize>>, course: usize, visited: &mut Vec<i32>) -> bool {
        if visited[course] == 1 {
            return true;
        }
        if visited[course] == -1 {
            return false;
        }
        visited[course] = -1;
        for p in &prereq[course] {
            if !Solution::dfs(prereq, *p, visited) {
                return false;
            }
        }
        visited[course] = 1;
        true
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut prereq: Vec<Vec<usize>> = vec![Vec::new(); num_courses as usize];
        for pre in prerequisites {
            prereq[pre[0] as usize].push(pre[1] as usize);
        }
        let mut visited = vec![0; num_courses as usize];
        for i in 0..num_courses as usize {
            if visited[i] == 0 {
                if !Solution::dfs(&prereq, i, &mut visited) {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
