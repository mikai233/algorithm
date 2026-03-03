struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut columns = [0u16; 9];
        let mut boxm = [0u16; 9];
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                let d = (c as u8 - b'1') as usize;
                let bit = 1u16 << d;
                let bi = (i / 3) * 3 + j / 3;
                if rows[i] & bit != 0 || columns[j] & bit != 0 || boxm[bi] & bit != 0 {
                    return false;
                }
                rows[i] |= bit;
                columns[j] |= bit;
                boxm[bi] |= bit;
            }
        }
        true
    }
}

fn main() {
    let board = [
        ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ];
    let board = board
        .iter()
        .map(|b| b.map(|c| c.chars().next().unwrap()).to_vec())
        .collect::<Vec<_>>();
    let valid = Solution::is_valid_sudoku(board);
    println!("{valid}");
}
