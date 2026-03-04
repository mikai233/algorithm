// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

use crate::common::solution::Solution;

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, _: i32) -> bool {
        unimplemented!()
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1i64;
        let mut r = n as i64 + 1;
        while l < r {
            let mid = l + (r - l) / 2;
            if self.isBadVersion(mid as i32) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}
