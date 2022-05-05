struct Solution;

struct MyStack {
    l: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { l: Vec::new() }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.l.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.l.pop().unwrap()
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.l.last().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.l.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    println!("Hello, world!");
}
