use std::collections::BinaryHeap;
struct MyCalendar {
    events: BinaryHeap<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let l = self.events.clone().into_sorted_vec();
        for (s, e) in l {
            if e - 1 < start {
                continue;
            }
            if s > end - 1 {
                break;
            }
            if (s <= start && start < e)
                || (s <= end && end < e)
                || (start <= s && s < end)
                || (start <= e && e < end)
            {
                return false;
            }
        }
        self.events.push((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

fn main() {
    let mut c = MyCalendar::new();
    let events = vec![
        (97, 100),
        (33, 51),
        (89, 100),
        (83, 100),
        (75, 92),
        (76, 95),
        (19, 30),
        (53, 63),
        (8, 23),
        (18, 37),
        (87, 100),
        (83, 100),
        (54, 67),
        (35, 48),
        (58, 75),
        (70, 89),
        (13, 32),
        (44, 63),
        (51, 62),
        (2, 15),
    ];
    for (s, e) in events {
        println!("{}", c.book(s, e));
    }
    println!("{:?}", c.events.into_sorted_vec());
}
