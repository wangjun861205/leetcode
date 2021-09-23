struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 'N';
        for _ in 0..4 {
            for inst in instructions.chars() {
                match inst {
                    'G' => match dir {
                        'N' => y += 1,
                        'S' => y -= 1,
                        'W' => x -= 1,
                        'E' => x += 1,
                        _ => unreachable!(),
                    },
                    'L' => match dir {
                        'N' => dir = 'W',
                        'S' => dir = 'E',
                        'W' => dir = 'S',
                        'E' => dir = 'N',
                        _ => unreachable!(),
                    },
                    'R' => match dir {
                        'N' => dir = 'E',
                        'S' => dir = 'W',
                        'W' => dir = 'N',
                        'E' => dir = 'S',
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            if x == 0 && y == 0 {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
