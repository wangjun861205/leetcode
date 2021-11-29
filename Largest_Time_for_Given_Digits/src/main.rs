struct Solution;

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut hour = -1;
        let mut minute = -1;
        for i in 0..4 {
            let mut l = arr.clone();
            let h1 = l.remove(i);
            for j in 0..3 {
                let mut ll = l.clone();
                let h2 = ll.remove(j);
                for k in 0..2 {
                    let mut lll = ll.clone();
                    let m1 = lll.remove(k);
                    let h = h1 * 10 + h2;
                    let m = m1 * 10 + lll[0];
                    if h < 24 && m < 60 {
                        if h > hour || h == hour && m > minute {
                            hour = h;
                            minute = m;
                        }
                    }
                }
            }
        }
        if hour == -1 {
            return "".to_owned();
        }
        format!("{:02}:{:02}", hour, minute)
    }
}

fn main() {
    println!("{}", Solution::largest_time_from_digits(vec![5, 5, 5, 5]));
}
