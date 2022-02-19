struct Solution;

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();
        let min_house = *houses.first().unwrap();
        let max_house = *houses.last().unwrap();
        let min_heater = *heaters.first().unwrap();
        let max_heater = *heaters.last().unwrap();
        let mut min = 0;
        let mut max = (min_heater - min_house)
            .abs()
            .max(min_heater - max_house.abs())
            .max((max_heater - min_house).abs())
            .max((max_heater - max_house).abs());
        'outer: while min < max {
            let mid = (min + max) / 2;
            let mut houses = houses.clone();
            let mut heaters = heaters.clone();
            let mut house = houses.remove(0);
            let mut heater = heaters.remove(0);
            loop {
                while house >= heater - mid && house <= heater + mid {
                    if houses.is_empty() {
                        max = mid;
                        continue 'outer;
                    }
                    house = houses.remove(0);
                }
                if house < heater - mid {
                    min = mid + 1;
                    continue 'outer;
                }
                if heaters.is_empty() {
                    min = mid + 1;
                    continue 'outer;
                }
                heater = heaters.remove(0);
            }
        }
        min
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_radius(
            vec![
                282475249, 622650073, 984943658, 144108930, 470211272, 101027544, 457850878,
                458777923
            ],
            vec![
                823564440, 115438165, 784484492, 74243042, 114807987, 137522503, 441282327,
                16531729, 823378840, 143542612
            ]
        )
    );
}
