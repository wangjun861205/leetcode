struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl Solution {
    fn deser(chars: &mut Vec<char>) -> NestedInteger {
        let mut vals: Vec<NestedInteger> = Vec::new();
        let mut val_str = String::new();
        while !chars.is_empty() {
            let c = chars.remove(0);
            match c {
                '[' => {
                    let next = Solution::deser(chars);
                    vals.push(next);
                }
                ']' => {
                    if !val_str.is_empty() {
                        vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
                        val_str.clear();
                    }
                    return NestedInteger::List(vals);
                }
                ',' => {
                    if !val_str.is_empty() {
                        vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
                        val_str.clear();
                    }
                }
                _ => {
                    val_str.push(c);
                }
            }
        }
        if !val_str.is_empty() {
            vals.push(NestedInteger::Int(val_str.clone().parse::<i32>().unwrap()));
            val_str.clear();
        }
        if vals.len() == 1 {
            vals.pop().unwrap()
        } else {
            NestedInteger::List(vals)
        }
    }
    pub fn deserialize(s: String) -> NestedInteger {
        Solution::deser(&mut s.chars().collect())
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::deserialize("[123,456,[788,799,833],[[]],10,[]]".to_owned())
    );
}
