// DFS to find CC returns x. |x| = |v| and gives the CC id for each vertex
// Takes the edges from prims as input and the number of total number of verticies
// use grpahix.num_vertices to get
pub fn dfs<K>(num_vertices: usize, edges: &[(usize, usize, K, usize)]) -> Vec<isize> {
    //build enighbor list for input edges
    let mut nbrs: Vec<Vec<usize>> = vec![Vec::new(); num_vertices];
    for &(u, v, _, _) in edges {
        nbrs[u].push(v);
        nbrs[v].push(u);
    }

    //DFS
    let mut x = vec![-1isize; num_vertices];
    let mut cc_id = 0isize;
    let mut stack = Vec::new();

    for start in 0..num_vertices {
        if x[start] != -1 {
            continue;
        }

        x[start] = cc_id;
        stack.push(start);

        while let Some(u) = stack.pop() {
            for &v in &nbrs[u] {
                if x[v] == -1 {
                    x[v] = cc_id;
                    stack.push(v);
                }
            }
        }
        cc_id += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::dfs;

    #[test]
    fn test_dfs_two_components() {
        let num_vertices = 5;
        let edges: Vec<(usize, usize, i32, usize)> = vec![
            (0, 1, 1, 0), // component 0: 0–1–2
            (1, 2, 1, 1),
            (3, 4, 1, 2), // component 1: 3–4
        ];
        let x = dfs(num_vertices, &edges);
        assert_eq!(x.len(), num_vertices);

        // 0,1,2 all share the same CC id
        assert_eq!(x[0], x[1]);
        assert_eq!(x[1], x[2]);

        // 3 and 4 share a different CC id
        assert_eq!(x[3], x[4]);
        assert_ne!(x[0], x[3]);
    }

    #[test]
    fn test_dfs_line_graph() {
        let num_vertices = 4;
        let edges: Vec<(usize, usize, i32, usize)> = vec![
            (0, 1, 5, 0), // 0–1–2–3 all one component
            (1, 2, 6, 1),
            (2, 3, 7, 2),
        ];
        let x = dfs(num_vertices, &edges);
        // All vertices must have the same CC id (0)
        assert_eq!(x, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_dfs_isolated_vertices() {
        let num_vertices = 3;
        let edges: Vec<(usize, usize, i32, usize)> = Vec::new();
        let x = dfs(num_vertices, &edges);
        // No edges → each vertex is its own component: 0,1,2
        assert_eq!(x, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_self_loop_and_multiple_ccs() {
        let num_vertices = 6;
        let edges: Vec<(usize, usize, i32, usize)> = vec![
            (0, 1, 1, 0), // CC0: {0,1}
            (2, 3, 1, 1), // CC1: {2,3}
            (3, 2, 1, 1), // duplicate in reverse order
            (4, 4, 1, 2), // self-loop at 4 → CC2: {4}
                          // vertex 5 never appears → CC3: {5}
        ];
        let x = dfs(num_vertices, &edges);

        // Check CC sizes and distinctions:
        assert_eq!(x[0], x[1]); // {0,1}
        assert_eq!(x[2], x[3]); // {2,3}
        assert_ne!(x[0], x[2]);
        assert_eq!(x[4], 2); // self-loop is its own CC
        assert_eq!(x[5], 3); // isolated vertex
    }
}
