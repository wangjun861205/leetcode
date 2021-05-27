struct FrontMiddleBackQueue {
    first_half: Vec<i32>,
    second_half: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            first_half: Vec::new(),
            second_half: Vec::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        if self.first_half.len() == self.second_half.len() {
            self.first_half.insert(0, val);
        } else {
            self.first_half.insert(0, val);
            self.second_half.insert(0, self.first_half.pop().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.first_half.len() == self.second_half.len() {
            self.first_half.push(val);
        } else {
            self.second_half.insert(0, self.first_half.pop().unwrap());
            self.first_half.push(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        if self.first_half.len() == 0 && self.second_half.len() == 0 {
            self.first_half.push(val);
        } else if self.first_half.len() == self.second_half.len() {
            self.second_half.push(val);
            self.first_half.push(self.second_half.remove(0));
        } else {
            self.second_half.push(val);
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.first_half.len() == 0 {
            return -1;
        }
        if self.first_half.len() == self.second_half.len() {
            self.first_half.push(self.second_half.remove(0));
            return self.first_half.remove(0);
        } else {
            return self.first_half.remove(0);
        }
    }

    fn pop_middle(&mut self) -> i32 {
        if self.first_half.len() == 0 {
            return -1;
        }
        if self.first_half.len() == self.second_half.len() {
            let v = self.first_half.pop().unwrap();
            self.first_half.push(self.second_half.remove(0));
            return v;
        } else {
            return self.first_half.pop().unwrap();
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.first_half.len() == 0 {
            return -1;
        }
        if self.first_half.len() == self.second_half.len() {
            return self.second_half.pop().unwrap();
        } else {
            self.second_half.insert(0, self.first_half.pop().unwrap());
            return self.second_half.pop().unwrap();
        }
    }
}
fn main() {
    println!("Hello, world!");
}
