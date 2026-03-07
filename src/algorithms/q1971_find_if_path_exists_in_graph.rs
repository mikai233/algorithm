use crate::common::solution::Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0], edge[1]);
            graph[u as usize].push(v as usize);
            graph[v as usize].push(u as usize);
        }
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source as usize);

        while let Some(node) = queue.pop_front() {
            if node == destination as usize {
                return true;
            }
            for &next in &graph[node] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        false
    }
}
