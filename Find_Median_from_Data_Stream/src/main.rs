struct MedianFinder {
    list: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn binary_insert(&mut self, start: usize, end: usize, num: i32) {
        if start == end {
            if self.list[start] > num {
                self.list.insert(start, num);
            } else {
                self.list.insert(start + 1, num);
            }
            return;
        }
        let mid = start + (end - start) / 2;
        if self.list[mid] < num {
            self.binary_insert(mid + 1, end, num);
        } else if self.list[mid] > num {
            self.binary_insert(start, mid, num);
        } else {
            self.list.insert(mid + 1, num);
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.list.is_empty() {
            self.list.push(num);
            return;
        }
        self.binary_insert(0, self.list.len() - 1, num)
    }

    fn find_median(&self) -> f64 {
        if self.list.is_empty() {
            return 0.0;
        }
        let mid_idx = (self.list.len() - 1) / 2;
        if self.list.len() % 2 == 0 {
            return (self.list[mid_idx] + self.list[mid_idx + 1]) as f64 / 2.0;
        } else {
            return self.list[mid_idx] as f64;
        }
    }
}

fn main() {
    let mut finder = MedianFinder::new();
    finder.add_num(-1);
    finder.add_num(-2);
    finder.add_num(-3);
    println!("{:?}", finder.list);
    println!("{}", finder.find_median());
}
