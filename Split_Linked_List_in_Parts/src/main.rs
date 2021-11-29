struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    fn from_vec(mut nodes: Vec<Box<ListNode>>) -> Option<Box<ListNode>> {
        if nodes.is_empty() {
            return None;
        }
        let mut head = nodes.remove(0);
        let tail = Solution::from_vec(nodes);
        head.next = tail;
        Some(head)
    }
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut l = Vec::new();
        while let Some(mut node) = head {
            let next = node.next.take();
            l.push(node);
            head = next;
        }
        if l.len() <= k as usize {
            let mut res = vec![None; k as usize];
            for (i, n) in l.into_iter().enumerate() {
                res[i] = Some(n);
            }
            return res;
        }
        if l.len() % k as usize == 0 {
            let ll: Vec<Vec<Box<ListNode>>> =
                l.chunks(l.len() / k as usize).map(|v| v.to_vec()).collect();
            return ll.into_iter().map(|v| Solution::from_vec(v)).collect();
        } else {
            let mut remain = l.len() % k as usize;
            let base = l.len() / k as usize;
            let mut res = Vec::new();
            while !l.is_empty() {
                if remain > 0 {
                    res.push(Solution::from_vec(l.drain(0..base as usize + 1).collect()));
                    remain -= 1;
                } else {
                    res.push(Solution::from_vec(l.drain(0..base as usize).collect()));
                }
            }
            return res;
        }
    }
}
fn main() {
    println!("Hello, world!");
}
