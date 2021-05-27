#![feature(refcell_take)]
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;
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

fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut first_odd: Rc<RefCell<Option<Box<ListNode>>>> = Rc::new(RefCell::new(None));
    let mut last_odd: Rc<RefCell<Option<Box<ListNode>>>> = Rc::new(RefCell::new(None));
    let mut first_even: Rc<RefCell<Option<Box<ListNode>>>> = Rc::new(RefCell::new(None));
    let mut last_even: Rc<RefCell<Option<Box<ListNode>>>> = Rc::new(RefCell::new(None));
    let mut curr: Rc<RefCell<Option<Box<ListNode>>>> = Rc::new(RefCell::new(head));
    let mut idx = 1;
    while curr.borrow().is_some() {
        let mut next: Option<Box<ListNode>> = None;
        swap(&mut curr.borrow_mut().as_mut().unwrap().next, &mut next);
        if idx % 2 == 1 {
            if first_odd.borrow().is_none() {
                first_odd = Rc::clone(&curr);
                last_odd = Rc::clone(&curr);
            } else {
                last_odd.borrow_mut().as_mut().unwrap().next = curr.borrow().clone();
                last_odd = Rc::clone(&curr);
            }
        } else {
            if first_even.borrow().is_none() {
                first_even = Rc::clone(&curr);
                last_even = Rc::clone(&curr);
            } else {
                last_even.borrow_mut().as_mut().unwrap().next = curr.borrow().clone();
                last_even = Rc::clone(&curr);
            }
        }
        curr = Rc::new(RefCell::new(next));
        idx += 1;
    }
    last_odd.borrow_mut().as_mut().unwrap().next = first_odd.borrow().clone();
    Some(first_odd.take().unwrap())
}

fn main() {
    let mut l = ListNode::new(1);
    let mut last = &mut l;
    for i in 2..6 {
        last.next = Some(Box::new(ListNode::new(i)));
        last = last.next.as_mut().unwrap().as_mut();
    }
    let mut ans = odd_even_list(Some(Box::new(l)));
    while let Some(node) = ans {
        println!("{:?}", node);
        ans = node.next;
    }
}
