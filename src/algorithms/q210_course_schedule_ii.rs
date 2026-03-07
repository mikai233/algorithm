use crate::common::solution::Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];
        for edge in prerequisites {
            let course = edge[0] as usize;
            let pre = edge[1] as usize;
            graph[pre].push(course);
            indegree[course] += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut order = vec![];
        while let Some(node) = queue.pop_front() {
            order.push(node as i32);
            for &next in &graph[node] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }
        if order.len() == n { order } else { vec![] }
    }
}
