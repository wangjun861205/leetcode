struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        if len1 == 0 && len2 == 0 {
            return 0_f64;
        }
        if len1 == 0 {
            if len2 % 2 == 0 {
                return (nums2[len2 / 2] + nums2[len2 / 2 - 1]) as f64 / 2_f64;
            }
            return nums2[len2 / 2] as f64;
        }
        if len2 == 0 {
            if len1 % 2 == 0 {
                return (nums1[len1 / 2] + nums1[len1 / 2 - 1]) as f64 / 2_f64;
            }
            return nums1[len1 / 2] as f64;
        }
        if len1 == 1 && len2 == 1 {
            return (nums1[0] + nums2[0]) as f64 / 2_f64;
        }
        let m1 = nums1[len1 / 2];
        let m2 = nums2[len2 / 2];
        if m1 < m2 {
            return Solution::find_median_sorted_arrays(
                nums1[len1 / 2..].to_vec(),
                nums2[..len2 / 2].to_vec(),
            );
        } else if m1 == m2 {
            return (m1 + m2) as f64 / 2_f64;
        } else {
            return Solution::find_median_sorted_arrays(
                nums1[..=len1 / 2].to_vec(),
                nums2[len2 / 2..].to_vec(),
            );
        }
    }
}

// 6, 7, 8, 9, 10
// 1, 2, 3, 4, 5
fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 8, 9, 10, 11, 12, 13])
    );
}
