use crate::common::solution::Solution;

#[derive(Debug, Default)]
struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn insert(&mut self, word: String) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(Default::default());
        }
        node.word = Some(word);
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut node = TrieNode::default();
        for word in words {
            node.insert(word);
        }
        fn dfs(
            board: &mut [Vec<char>],
            i: i32,
            j: i32,
            node: &mut TrieNode,
            ans: &mut Vec<String>,
        ) {
            let m = board.len() as i32;
            let n = board[0].len() as i32;
            if i < 0 || i >= m || j < 0 || j >= n {
                return;
            }
            let (ui, uj) = (i as usize, j as usize);
            let curr = board[ui][uj];
            if curr == '#' {
                return;
            }
            match node.children.get_mut(&curr) {
                Some(child) => {
                    board[ui][uj] = '#';
                    if let Some(word) = child.word.take() {
                        ans.push(word);
                    }
                    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dx, dy) in dirs {
                        dfs(board, i + dx, j + dy, child, ans);
                    }
                    board[ui][uj] = curr;
                }
                None => {
                    return;
                }
            }
        }
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let mut ans = vec![];
        for i in 0..m {
            for j in 0..n {
                dfs(&mut board, i, j, &mut node, &mut ans);
            }
        }
        ans
    }
}
