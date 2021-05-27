struct Solution;

impl Solution {
    fn even_m_to_c(x: usize, y: usize, middle: f32) -> (i32, i32) {
        (
            if x as f32 - middle < 0.0 {
                (x as f32 - middle - 0.5) as i32
            } else {
                (x as f32 - middle + 0.5) as i32
            },
            if y as f32 - middle < 0.0 {
                (y as f32 - middle - 0.5) as i32
            } else {
                (y as f32 - middle + 0.5) as i32
            },
        )
    }

    fn even_c_to_m(x: i32, y: i32, middle: f32) -> (usize, usize) {
        (
            if x < 0 {
                (x as f32 + 0.5 + middle) as usize
            } else {
                (x as f32 - 0.5 + middle) as usize
            },
            if y < 0 {
                (y as f32 + 0.5 + middle) as usize
            } else {
                (y as f32 - 0.5 + middle) as usize
            },
        )
    }

    fn odd_m_to_c(x: usize, y: usize, middle: f32) -> (i32, i32) {
        ((x as f32 - middle) as i32, (y as f32 - middle) as i32)
    }

    fn odd_c_to_m(x: i32, y: i32, middle: f32) -> (usize, usize) {
        ((x as f32 + middle) as usize, (y as f32 + middle) as usize)
    }
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let m_to_c = if len % 2 == 0 {
            Solution::even_m_to_c
        } else {
            Solution::odd_m_to_c
        };
        let c_to_m = if len % 2 == 0 {
            Solution::even_c_to_m
        } else {
            Solution::odd_c_to_m
        };
        let middle: f32 = len as f32 / 2.0 - 0.5;
        for x in 0..len {
            for y in 0..len {
                let (mut cx, mut cy) = m_to_c(x, y, middle);
                if len % 2 == 0 {
                    if cx <= 0 && cy <= 0 {
                        let mut value = matrix[y][x];
                        for _ in 0..4 {
                            let rcx = -cy;
                            let rcy = cx;
                            let (rmx, rmy) = c_to_m(rcx, rcy, middle);
                            let tmp = matrix[rmy][rmx];
                            matrix[rmy][rmx] = value;
                            cx = rcx;
                            cy = rcy;
                            value = tmp;
                        }
                    }
                } else {
                    if cx <= 0 && cy < 0 {
                        let mut value = matrix[y][x];
                        for _ in 0..4 {
                            let rcx = -cy;
                            let rcy = cx;
                            let (rmx, rmy) = c_to_m(rcx, rcy, middle);
                            let tmp = matrix[rmy][rmx];
                            matrix[rmy][rmx] = value;
                            cx = rcx;
                            cy = rcy;
                            value = tmp;
                        }
                    }
                }
            }
        }
    }
}
fn main() {
    let mut v = vec![vec![5, 1, 9], vec![2, 4, 8], vec![13, 3, 6]];
    Solution::rotate(&mut v);
    println!("{:?}", v)
}
