struct Solution;

#[derive(Debug)]
struct Node {
    val: i32,
    children: Box<Vec<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            children: Box::new(Vec::new()),
        }
    }

    fn add(&mut self, num: i32) {
        let mut is_added = false;
        for c in self.children.as_mut() {
            if c.val % num == 0 || num % c.val == 0 {
                is_added = true;
                c.add(num);
            }
        }
        if !is_added {
            self.children.push(Node::new(num));
        }
    }

    fn to_vec(&self) -> Vec<Vec<i32>> {
        if self.children.is_empty() {
            return vec![vec![self.val]];
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for c in self.children.as_ref() {
            let l = c.to_vec();
            for mut ll in l {
                ll.insert(0, self.val);
                ans.push(ll);
            }
        }
        ans
    }
}

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut root = Node::new(-1);
        for n in nums {
            root.add(n);
        }
        println!("{:?}", root);
        let l = root.to_vec();
        println!("{:?}", l);
        let mut ans = l.into_iter().max_by_key(|ll| ll.len()).unwrap();
        ans.remove(0);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::largest_divisible_subset(vec![3, 17]));
}
