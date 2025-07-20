use std::cmp::min;

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len % 2 == 0 {
            let e1 = Solution::find_k_element(&*nums1, &*nums2, len / 2) as f64;
            let e2 = Solution::find_k_element(&nums1, &nums2, len / 2 + 1) as f64;
            (e1 + e2) / 2.0
        } else {
            Solution::find_k_element(&nums1, &nums2, len / 2 + 1) as f64
        }
    }

    fn find_k_element<'a>(mut nums1: &'a [i32], mut nums2: &'a [i32], mut k: usize) -> i32 {
        loop {
            if nums1.len() > nums2.len() {
                std::mem::swap(&mut nums1, &mut nums2);
            }
            if nums1.is_empty() {
                return nums2[k - 1];
            }
            if k == 1 {
                return min(nums1[0], nums2[0]);
            }
            let i = min(nums1.len(), k / 2);
            let j = k - i;
            if nums1[i - 1] < nums2[j - 1] {
                nums1 = &nums1[i..];
                k -= i;
            } else {
                nums2 = &nums2[j..];
                k -= j;
            }
        }
    }
}
fn main() {
    println!("{}", Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    println!("{}", Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}