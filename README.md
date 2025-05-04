# graph-dfs

[![crates.io](https://img.shields.io/crates/v/graph-dfs.svg)](https://crates.io/crates/graph-dfs)
[![docs.rs](https://docs.rs/graph-dfs/badge.svg)](https://docs.rs/graph-dfs)

A minimal, dependency-free crate to compute connected components via iterative DFS on any CSR-style graph.

## Features

- **Generic** `GraphAccess` trait for plugging in your own graph type
- **True** _O(V + E)_ connected-component labeling via iterative DFS
- **No dependencies** beyond `std` – works in `no_std` contexts
- **Zero allocations** during traversal aside from the output buffer

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
graph-dfs = "0.1"
````

## Quick Example

```rust
use graph_dfs::{GraphAccess, compute_cc_by_dfs};

/// A simple CSR-style graph:
struct MyGraph {
    /// CSR offsets: `v[u]..v[u+1]` are the half-edges from `u`
    v: Vec<usize>,
    /// half-edges: `(neighbor, weight, orig_eid)`
    e: Vec<(usize, usize, usize)>,
}

impl GraphAccess for MyGraph {
    type Weight = usize;

    fn num_vertices(&self) -> usize {
        self.v.len() - 1
    }

    fn edges_from(&self, u: usize) -> &[(usize, Self::Weight, usize)] {
        let start = self.v[u];
        let end   = self.v[u + 1];
        &self.e[start..end]
    }
}

fn main() {
    // build a tiny triangle graph: 0–1, 1–2, 2–0
    let v = vec![0, 2, 4, 6];
    let e = vec![
        (1, 1, 0), (2, 1, 2), // from 0
        (0, 1, 0), (2, 1, 1), // from 1
        (1, 1, 1), (0, 1, 2), // from 2
    ];
    let graph = MyGraph { v, e };

    // compute connected components
    let cc_ids = compute_cc_by_dfs(&graph);
    assert_eq!(cc_ids, vec![0, 0, 0]); // all in one component
}
```

## Documentation

See the full API docs on [docs.rs](https://docs.rs/graph-dfs).

## License

See the [LICENSE](./LICENSE) file for details.
