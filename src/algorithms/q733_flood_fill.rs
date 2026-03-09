use crate::common::solution::Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        fn fill(image: &mut [Vec<i32>], i: i32, j: i32, source: i32, target: i32) {
            let m = image.len() as i32;
            let n = image[0].len() as i32;
            if i < 0 || i >= m || j < 0 || j >= n {
                return;
            }
            let curr = image[i as usize][j as usize];
            if source != curr || source == target {
                return;
            }
            image[i as usize][j as usize] = target;
            fill(image, i, j - 1, source, target);
            fill(image, i, j + 1, source, target);
            fill(image, i - 1, j, source, target);
            fill(image, i + 1, j, source, target);
        }
        let source = image[sr as usize][sc as usize];
        fill(&mut image, sr, sc, source, color);
        image
    }
}
