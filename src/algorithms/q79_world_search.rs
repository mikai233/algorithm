use crate::common::solution::Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let words: Vec<char> = word.chars().collect();
        fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, words: &[char], k: usize) -> bool {
            if k == words.len() {
                return true;
            }
            let m = board.len() as i32;
            let n = board[0].len() as i32;
            if i < 0 || i >= m || j < 0 || j >= n {
                return false;
            }
            let x = i as usize;
            let y = j as usize;
            if board[x][y] != words[k] {
                return false;
            }
            board[x][y] = '#';
            let left = dfs(board, i - 1, j, words, k + 1);
            let right = dfs(board, i + 1, j, words, k + 1);
            let up = dfs(board, i, j - 1, words, k + 1);
            let down = dfs(board, i, j + 1, words, k + 1);
            let found = left || right || up || down;
            board[x][y] = words[k];
            found
        }
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        for i in 0..m {
            for j in 0..n {
                if dfs(&mut board, i, j, &words, 0) {
                    return true;
                }
            }
        }
        false
    }
}
