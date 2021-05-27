use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialOrd + Clone + Display,
{
    pub val: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: PartialOrd + Clone + Display,
{
    pub fn new(val: T) -> Node<T> {
        Node {
            val: val,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, val: T) {
        let node = Node {
            val: val.clone(),
            left: None,
            right: None,
        };
        if self.val < val.clone() {
            if let Some(right) = &self.right {
                right.borrow_mut().add(val);
            } else {
                self.right = Some(Rc::new(RefCell::new(node)));
            }
        } else {
            if let Some(left) = &self.left {
                left.borrow_mut().add(val);
            } else {
                self.left = Some(Rc::new(RefCell::new(node)));
            }
        }
    }
}

fn main() {
    let mut root = Node::new(8);
    root.add(3);
    root.add(4);
    println!("{:?}", root.val);
    println!("{:?}", root.left.as_ref().unwrap().borrow().val);
    println!(
        "{:?}",
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .val
    );
}
