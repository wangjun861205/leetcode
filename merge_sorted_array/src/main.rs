struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut total = m + n;
        let mut ns1 = nums1[..m as usize].to_vec();
        let mut ns2 = nums2[..n as usize].to_vec();
        let mut ns3: Vec<i32> = Vec::new();
        while total > 0 {
            if ns1.len() == 0 {
                ns3.append(&mut ns2);
                break;
            }
            if ns2.len() == 0 {
                ns3.append(&mut ns1);
                break;
            }
            if ns1[0] < ns2[0] {
                ns3.push(ns1.remove(0));
            } else if ns1[0] > ns2[0] {
                ns3.push(ns2.remove(0));
            } else {
                ns3.push(ns1.remove(0));
                ns3.push(ns2.remove(0));
            }
            total -= 1;
        }
        *nums1 = ns3;
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 4];
    let mut nums2 = vec![3, 4, 5, 6, 100];
    Solution::merge(&mut nums1, 4, &mut nums2, 5);
    println!("{:?}", nums1)
}
