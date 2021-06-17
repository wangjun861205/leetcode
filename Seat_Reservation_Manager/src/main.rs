use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SeatManager {
    seats: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).into_iter().map(|v| Reverse(v)).collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.seats.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(Reverse(seat_number));
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */

fn main() {
    println!("Hello, world!");
}
