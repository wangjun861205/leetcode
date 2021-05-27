#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    list: Vec<NestedInteger>,
    inner_iter: Option<Box<NestedIterator>>,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        Self {
            list: nestedList,
            inner_iter: None,
        }
    }

    fn next(&mut self) -> i32 {
        if let Some(ref mut iter) = self.inner_iter {
            return iter.next();
        }
        match self.list.remove(0) {
            NestedInteger::Int(v) => v,
            NestedInteger::List(l) => {
                let mut iter = NestedIterator::new(l);
                let val = iter.next();
                self.inner_iter = Some(Box::new(iter));
                val
            }
        }
    }

    fn has_next(&mut self) -> bool {
        if let Some(ref mut iter) = self.inner_iter {
            if iter.has_next() {
                return true;
            }
            self.inner_iter = None;
        }
        if self.list.len() == 0 {
            return false;
        }
        while self.list.len() > 0 {
            match self.list.remove(0) {
                NestedInteger::Int(v) => {
                    self.list.insert(0, NestedInteger::Int(v));
                    return true;
                }
                NestedInteger::List(l) => {
                    let mut iter = NestedIterator::new(l);
                    if !iter.has_next() {
                        continue;
                    }
                    self.inner_iter = Some(Box::new(iter));
                    return true;
                }
            }
        }
        false
    }
}
fn main() {
    println!("Hello, world!");
}
