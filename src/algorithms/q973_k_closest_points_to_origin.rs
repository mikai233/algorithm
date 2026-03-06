use crate::common::solution::Solution;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if k == points.len() {
            return points;
        }

        let target = k - 1;
        let n = points.len();
        let (mut left, mut right) = (0usize, n - 1);

        while left <= right {
            let (lt, gt) = Solution::partition3_2(&mut points, left, right);

            if target < lt {
                right = lt - 1;
            } else if target > gt {
                left = gt + 1;
            } else {
                break;
            }
        }

        points[..k].to_vec()
    }

    fn partition3_2(points: &mut Vec<Vec<i32>>, left: usize, right: usize) -> (usize, usize) {
        let mid = left + (right - left) / 2;
        points.swap(mid, right);

        let pivot_len = dist(&points[right]);

        let mut lt = left; // [left..lt) < pivot
        let mut i = left; // [lt..i) == pivot
        let mut gt = right; // (gt..right] > pivot

        while i <= gt {
            let d = dist(&points[i]);
            if d < pivot_len {
                points.swap(i, lt);
                lt += 1;
                i += 1;
            } else if d > pivot_len {
                points.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            } else {
                i += 1;
            }
        }

        (lt, gt)
    }
}

fn dist(p: &Vec<i32>) -> i64 {
    let x = p[0] as i64;
    let y = p[1] as i64;
    x * x + y * y
}
