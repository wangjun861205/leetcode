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
    fn append(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut l1) = list1 {
            if let Some(l2) = list2 {
                let next = l1.next.take();
                l1.next = Solution::append(next, Some(l2));
            }
            return Some(l1);
        }
        list2
    }

    fn trim(list: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            return list;
        }
        if let Some(mut l) = list {
            let next = l.next.take();
            return Solution::trim(next, n - 1);
        }
        None
    }

    fn merge(
        mut prev: Option<Box<ListNode>>,
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
        curr: i32,
    ) -> Option<Box<ListNode>> {
        if curr == a {
            let remain = Solution::trim(list1, b - a + 1);
            let tail = Solution::append(list2, remain);
            return Solution::append(prev, tail);
        }
        if let Some(mut l1) = list1 {
            let next = l1.next.take();
            prev = Solution::append(prev, Some(l1));
            return Solution::merge(prev, next, a, b, list2, curr + 1);
        }
        None
    }

    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::merge(None, list1, a, b, list2, 0)
    }
}
fn main() {
    println!("Hello, world!");
}
