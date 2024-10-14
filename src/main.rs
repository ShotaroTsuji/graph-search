use graph_search::{AdjVec, AdjacentVertices};

fn main() {
    let adj = vec![
        vec![1, 10],            // 0
        vec![0, 2, 11],         // 1
        vec![1, 3, 12],         // 2
        vec![2, 4, 12, 13],     // 3
        vec![3, 5],             // 4
        vec![4, 6, 8, 12],      // 5
        vec![5, 7],             // 6
        vec![6, 8],             // 7
        vec![5, 7, 9, 13],      // 8
        vec![8, 10, 11, 12, 13],// 9
        vec![0, 9, 11],         // 10
        vec![1, 9, 10, 12],     // 11
        vec![2, 3, 5, 9, 11, 13], // 12
        vec![3, 8, 9, 12],      // 13
    ];
    let adj = AdjVec::from_vec(adj).unwrap();
    assert!(adj.is_undirected());

    println!("## BFS");
    for v in adj.bfs(0) {
        println!("{}", v);
    }

    println!("");
    println!("## DFS");
    for v in adj.dfs(0) {
        println!("{}", v);
    }
}
