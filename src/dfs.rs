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
