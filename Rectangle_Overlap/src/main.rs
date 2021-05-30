struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (start_x_1, start_y_1, end_x_1, end_y_1) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (start_x_2, start_y_2, end_x_2, end_y_2) = (rec2[0], rec2[1], rec2[2], rec2[3]);
        !(start_x_1 >= end_x_2
            || end_x_1 <= start_x_2
            || start_y_1 >= end_y_2
            || end_y_1 <= start_y_2)
            && (start_x_1 != end_x_1)
            && (start_y_1 != end_y_1)
            && (start_x_2 != end_x_2)
            && (start_y_2 != end_y_2)
    }
}
fn main() {
    println!("Hello, world!");
}
