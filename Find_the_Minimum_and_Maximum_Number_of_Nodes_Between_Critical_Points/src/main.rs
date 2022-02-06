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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut first = head;
        let mut second = first.as_mut().unwrap().next.take();
        let mut remain = second.as_mut().unwrap().next.take();
        let mut min = i32::MAX;
        let mut max = -1;
        let mut head: Option<i32> = None;
        let mut prev: Option<i32> = None;
        let mut idx = 0;
        while remain.is_some() {
            let first_value = first.unwrap().val;
            let second_value = second.as_ref().unwrap().val;
            let third_value = remain.as_ref().unwrap().val;
            let rem = remain.as_mut().unwrap().next.take();
            if second_value < first_value && second_value < third_value
                || second_value > first_value && second_value > third_value
            {
                if let Some(p) = &mut prev {
                    min = min.min(idx - *p);
                    *p = idx;
                } else {
                    prev = Some(idx);
                }
                if let Some(h) = head {
                    max = max.max(idx - h);
                } else {
                    head = Some(idx);
                }
            }
            idx += 1;
            first = second;
            second = remain;
            remain = rem;
        }
        vec![if min == i32::MAX { -1 } else { min }, max]
    }
}

fn main() {
    println!("Hello, world!");
}
