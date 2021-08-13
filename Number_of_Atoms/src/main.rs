struct Solution;

use std::collections::HashMap;

impl Solution {
    fn read_elem(mut formula: Vec<char>) -> ((String, i32), Vec<char>) {
        let mut s = String::new();
        let mut c = String::new();
        s.push(formula.remove(0));
        while let Some(&fst) = formula.first() {
            if fst.is_lowercase() {
                s.push(formula.remove(0));
            } else if fst.is_numeric() {
                c.push(formula.remove(0));
            } else {
                break;
            }
        }
        if c.is_empty() {
            ((s, 1), formula)
        } else {
            ((s, c.parse().unwrap()), formula)
        }
    }

    fn read_group_count(mut formula: Vec<char>) -> (i32, Vec<char>) {
        let mut c = String::new();
        while let Some(&fst) = formula.first() {
            if fst.is_numeric() {
                c.push(formula.remove(0));
            } else {
                break;
            }
        }
        if c.is_empty() {
            (1, formula)
        } else {
            (c.parse().unwrap(), formula)
        }
    }

    fn parse(mut formula: Vec<char>) -> (HashMap<String, i32>, Vec<char>) {
        let mut elems = HashMap::new();
        while let Some(&fst) = formula.first() {
            if fst == '(' {
                formula.remove(0);
                let (m, f) = Solution::parse(formula);
                m.into_iter().for_each(|(k, v)| {
                    *elems.entry(k).or_insert(0) += v;
                });
                formula = f;
            } else if fst == ')' {
                formula.remove(0);
                let (count, f) = Solution::read_group_count(formula);
                if count > 1 {
                    elems.values_mut().for_each(|c| *c *= count);
                }
                formula = f;
                break;
            } else if fst.is_uppercase() {
                let ((s, c), f) = Solution::read_elem(formula);
                *elems.entry(s).or_insert(0) += c;
                formula = f;
            }
        }
        (elems, formula)
    }

    pub fn count_of_atoms(formula: String) -> String {
        let (m, _) = Solution::parse(formula.chars().collect());
        let mut l: Vec<(String, i32)> = m.into_iter().collect();
        l.sort();
        l.into_iter()
            .map(|(s, c)| if c == 1 { s } else { format!("{}{}", s, c) })
            .collect()
    }
}

fn main() {
    println!("{}", Solution::count_of_atoms("Mg(OH)2".to_owned()));
}
