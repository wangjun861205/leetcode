struct Solution;
impl Solution {
    pub fn get_permutation(mut n: i32, mut k: i32) -> String {
        let mut l: Vec<i32> = (1..=n).into_iter().collect();
        let factorial: Vec<i32> = l
            .iter()
            .scan(1, |s, v| {
                *s *= *v;
                Some(*s)
            })
            .collect();
        println!("{:?}", l);
        println!("{:?}", &factorial);
        let mut ans: Vec<i32> = Vec::new();
        while n > 1 {
            ans.push(l.remove((k / factorial[n as usize - 2]) as usize));
            k = k % factorial[n as usize - 2];
            n -= 1;
        }
        ans.push(l[0]);
        ans.into_iter().map(|v| v.to_string()).collect()
    }
}
fn main() {
    println!("{}", Solution::get_permutation(4, 9));
}
