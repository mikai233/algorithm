struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = nums1.len();
        while i > 0 && j > 0 {
            if nums1[i - 1] < nums2[j - 1] {
                nums1[k - 1] = nums2[j - 1];
                j -= 1;
            } else {
                nums1[k - 1] = nums1[i - 1];
                i -= 1;
            }
            k -= 1;
        }
        while j > 0 {
            nums1[k - 1] = nums2[j - 1];
            j -= 1;
            k -= 1;
        }
    }
}

fn main() {
    let mut v1 = vec![1, 2, 3, 0, 0, 0];
    let mut v2 = vec![2, 5, 6];
    Solution::merge(&mut v1, 3, &mut v2, 3);
    println!("{v1:?}");
    let mut v1 = vec![1];
    let mut v2 = vec![];
    Solution::merge(&mut v1, 1, &mut v2, 0);
    println!("{v1:?}");
    let mut v1 = vec![0];
    let mut v2 = vec![1];
    Solution::merge(&mut v1, 0, &mut v2, 1);
    println!("{v1:?}");
    let mut v1 = vec![2, 0];
    let mut v2 = vec![1];
    Solution::merge(&mut v1, 1, &mut v2, 1);
    println!("{v1:?}");
}