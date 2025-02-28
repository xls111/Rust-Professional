/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = Vec::new(); // 记录访问顺序
        let mut visited = vec![false; self.adj.len()]; // 标记节点是否已访问
        let mut queue = VecDeque::new(); // 使用 VecDeque 作为队列

        // 将起始节点加入队列并标记为已访问
        queue.push_back(start);
        visited[start] = true;

        // 开始 BFS
        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 记录当前节点到访问顺序中

            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记邻居为已访问
                    queue.push_back(neighbor); // 将邻居加入队列
                }
            }
        }

        visit_order
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return2(&self, start: usize) -> Vec<usize> {
        //TODO
        let mut nodes: Vec<usize> = vec![];
        let mut visit_order = vec![];
        let mut flags = vec![];
        for _i in 0..self.adj.len(){
            flags.push(0);
        }
        nodes.push(start);

        loop {
            if 0 == nodes.len() {
                break;
            } else {
                let data = nodes.remove(0);
                if !visit_order.contains(&data) {
                    visit_order.push(data);
                }

                if 0 == flags[data] {
                    flags[data] = 1;
                    for d in &self.adj[data] {
                        if !visit_order.contains(d) {
                            nodes.push(*d);
                        }
                    }
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
