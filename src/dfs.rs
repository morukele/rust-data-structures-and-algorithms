/**
 * Question:
 * You are given an undirected graph with N nodes labeled from 0 to N-1 and a list of edges.
 * Your task is to find the number of connected components in the graph using the DFS algorithm.
 * A connected component is a group of nodes where each node is reachable from any other node in the same group.
 */
pub fn count_components(n: usize, edges: Vec<Vec<usize>>) -> i32 {
    // Step 1: build the adjacency list
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0];
        let v = edge[1];

        // since it is undirected, add all possible connections to the graph
        graph[u].push(v);
        graph[v].push(u);
    }

    // Depth first search
    let mut visited = vec![false; n]; // using the n as index
    let mut components = 0;

    for i in 0..n {
        if !visited[i] {
            components += 1;
            dfs(i, &graph, &mut visited);
        }
    }

    components
}

pub fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[node] = true; // set node as visited
    for &neigbhour in &graph[node] {
        // check the neigbhours of the node
        if !visited[neigbhour] {
            // if the neigbhour has not been visited, do recursive dfs
            dfs(neigbhour, graph, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![2, 3]];
        let output = count_components(n, edges);

        assert_eq!(output, 2);
    }

    #[test]
    fn test_case_2() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let output = count_components(n, edges);

        assert_eq!(output, 1);
    }

    #[test]
    fn test_case_3() {
        let n = 3;
        let edges = vec![];
        let output = count_components(n, edges);

        assert_eq!(output, 3);
    }

    #[test]
    fn test_case_4() {
        let n = 7;
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4], vec![5, 6]];
        let output = count_components(n, edges);

        assert_eq!(output, 3);
    }

    #[test]
    fn test_case_5() {
        let n = 1;
        let edges = vec![];
        let output = count_components(n, edges);

        assert_eq!(output, 1);
    }
}
