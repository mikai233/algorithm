struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut res: Vec<Vec<i32>> = vec![];
        for ele in intervals {
            match res.last_mut() {
                Some(last) => {
                    if ele[0] > last[1] {
                        res.push(ele);
                    } else if ele[1] > last[1] {
                        last[1] = ele[1];
                    }
                }
                None => {
                    res.push(ele);
                }
            }
        }
        res
    }
}

fn main() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let ans = Solution::merge(intervals);
    println!("{ans:?}");
}
