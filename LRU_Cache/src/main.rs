use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key: key,
            val: val,
            prev: None,
            next: None,
        }
    }
    fn remove(&mut self) {
        let prev = self.prev.take();
        let next = self.next.take();
        if let Some(p) = prev.as_ref() {
            p.borrow_mut().next = next.clone();
        }
        if let Some(n) = next.as_ref() {
            n.borrow_mut().prev = prev.clone();
        }
    }
}

#[derive(Debug)]
struct DoubleLinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn add_to_head(&mut self, rf: Rc<RefCell<ListNode>>) {
        if self.head.is_none() && self.tail.is_none() {
            self.head = Some(rf);
        } else if self.head.is_none() && self.tail.is_some() {
            rf.borrow_mut().next = Some(self.tail.as_ref().unwrap().clone());
            self.tail.as_ref().unwrap().borrow_mut().prev = Some(rf.clone());
            self.head = Some(rf.clone());
        } else if self.head.is_some() && self.tail.is_none() {
            let head = self.head.take();
            rf.borrow_mut().next = head.clone();
            head.as_ref().unwrap().borrow_mut().prev = Some(rf.clone());
            self.tail = head.clone();
            self.head = Some(rf.clone());
        } else {
            let head = self.head.take();
            head.as_ref().unwrap().borrow_mut().prev = Some(rf.clone());
            rf.borrow_mut().next = head;
            self.head = Some(rf.clone());
        }
    }
    fn add_to_tail(&mut self, rf: Rc<RefCell<ListNode>>) {
        if self.head.is_none() && self.tail.is_none() {
            self.tail = Some(rf);
        } else if self.head.is_none() && self.tail.is_some() {
            let tail = self.tail.take();
            tail.as_ref().unwrap().borrow_mut().next = Some(rf.clone());
            rf.borrow_mut().prev = tail.clone();
            self.head = tail.clone();
            self.tail = Some(rf.clone());
        } else if self.head.is_some() && self.tail.is_none() {
            let head = self.head.take();
            head.as_ref().unwrap().borrow_mut().next = Some(rf.clone());
            rf.borrow_mut().prev = head.clone();
            self.head = head;
            self.tail = Some(rf.clone());
        } else {
            let tail = self.tail.take();
            tail.as_ref().unwrap().borrow_mut().next = Some(rf.clone());
            rf.borrow_mut().prev = tail;
            self.tail = Some(rf.clone());
        }
    }
    fn remove_head(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        } else if self.head.is_some() && self.tail.is_none() {
            return self.head.take();
        } else if self.head.is_none() && self.tail.is_some() {
            return self.tail.take();
        } else {
            let head = self.head.take();
            let second_head = head.as_ref().unwrap().borrow_mut().next.take();
            if second_head.as_ref().unwrap().as_ptr() == self.tail.as_ref().unwrap().as_ptr() {
                self.tail.as_ref().unwrap().borrow_mut().prev = None;
                return head;
            }
            second_head.as_ref().unwrap().borrow_mut().prev = None;
            head.as_ref().unwrap().borrow_mut().next = None;
            self.head = second_head;
            return head;
        }
    }

    fn remove_tail(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        } else if self.head.is_some() && self.tail.is_none() {
            return self.head.take();
        } else if self.head.is_none() && self.tail.is_some() {
            return self.tail.take();
        } else {
            let tail = self.tail.take();
            let second_tail = tail.as_ref().unwrap().borrow_mut().prev.take();
            if second_tail.as_ref().unwrap().as_ptr() == self.head.as_ref().unwrap().as_ptr() {
                self.head.as_ref().unwrap().borrow_mut().next = None;
                return tail;
            }
            second_tail.as_ref().unwrap().borrow_mut().next = None;
            tail.as_ref().unwrap().borrow_mut().prev = None;
            self.tail = second_tail;
            return tail;
        }
    }

    fn remove_node(&mut self, node: Rc<RefCell<ListNode>>) {
        if node.as_ptr() == self.head.as_ref().unwrap().as_ptr() {
            self.remove_head();
            return;
        }
        if node.as_ptr() == self.tail.as_ref().unwrap().as_ptr() {
            self.remove_tail();
            return;
        }
        node.borrow_mut().remove();
    }
}

struct LRUCache {
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    list: DoubleLinkedList,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            list: DoubleLinkedList::new(),
            cap: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(rf) = self.map.get_mut(&key) {
            self.list.remove_node(rf.clone());
            let new_node = Rc::new(RefCell::new(ListNode::new(
                rf.borrow().key,
                rf.borrow().val,
            )));
            self.list.add_to_head(new_node.clone());
            *rf = new_node;
            return rf.borrow().val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(rf) = self.map.get_mut(&key) {
            self.list.remove_node(rf.clone());
            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.list.add_to_head(new_node.clone());
            *rf = new_node;
            return;
        }
        if self.map.len() == self.cap {
            let v = self.list.remove_tail().unwrap().borrow().key;
            self.map.remove(&v);
        }
        let rf = Rc::new(RefCell::new(ListNode::new(key, value)));
        self.list.add_to_head(rf.clone());
        self.map.insert(key, rf.clone());
    }
}

#[cfg(test)]
mod test {
    use super::{DoubleLinkedList, ListNode};
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_double_linked_list() {
        let mut l = DoubleLinkedList::new();
        l.add_to_tail(Rc::new(RefCell::new(ListNode::new(1, 1))));
        l.add_to_tail(Rc::new(RefCell::new(ListNode::new(2, 2))));
        l.add_to_tail(Rc::new(RefCell::new(ListNode::new(3, 3))));
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        l.add_to_head(Rc::new(RefCell::new(ListNode::new(1, 1))));
        l.add_to_head(Rc::new(RefCell::new(ListNode::new(2, 2))));
        l.add_to_head(Rc::new(RefCell::new(ListNode::new(3, 3))));
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
    }
    #[test]
    fn test_remove() {
        let mut l = DoubleLinkedList::new();
        let node1 = Rc::new(RefCell::new(ListNode::new(1, 1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2, 2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(3, 3)));
        l.add_to_tail(node1.clone());
        l.add_to_tail(node2.clone());
        l.add_to_tail(node3.clone());
        node2.borrow_mut().remove();
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
        println!("{:?}", l.remove_tail());
    }
    #[test]
    fn test() {
        let mut l = DoubleLinkedList::new();
        let one = Rc::new(RefCell::new(ListNode::new(2, 6)));
        let two = Rc::new(RefCell::new(ListNode::new(2, 2)));
        let three = Rc::new(RefCell::new(ListNode::new(3, 3)));
        let four = Rc::new(RefCell::new(ListNode::new(4, 4)));
        l.add_to_head(one.clone());
        l.add_to_head(two.clone());
        l.remove_node(one.clone());
        println!("{}", l.remove_tail().unwrap().borrow().val);
        // println!("{}", l.remove_tail().unwrap().borrow().val);
        // l.add_to_head(one.clone());
        // two.borrow_mut().remove();
        // l.add_to_head(three);
        // one.borrow_mut().remove();
        // println!("{}", l.remove_tail().unwrap().borrow().val,);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn main() {
    let mut cache = LRUCache::new(3);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1);
    cache.put(3, 3);
    cache.get(2);
    cache.put(4, 4);
    cache.get(1);
    cache.get(3);
    cache.get(4);
}
