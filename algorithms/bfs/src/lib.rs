pub mod graph;




#[cfg(test)]
mod tests {

    use crate::graph::Graph;
    use std::collections::HashSet;


    #[test]
    fn test_bfs() {




        let mut graph = Graph::new();
        graph.add_vertice(1, HashSet::from([2, 5, 1]));
        graph.add_vertice(2, HashSet::from([3]));
        graph.add_vertice(3, HashSet::from([3, 4]));
        graph.add_vertice(4, HashSet::from([5]));
        graph.add_vertice(5, HashSet::from([6]));
        graph.add_vertice(6, HashSet::from([]));
        graph.add_vertice(7, HashSet::from([6]));
        graph.add_vertice(8, HashSet::from([7]));


        assert_eq!(graph.bfs(1, 5), true);
        assert_eq!(graph.bfs(1, 6), true);
        assert_eq!(graph.bfs(1, 7), false);
        assert_eq!(graph.bfs(8, 5), false);
        assert_eq!(graph.bfs(5, 1), false);
        assert_eq!(graph.bfs(1, 4), true);
        assert_eq!(graph.bfs(1, 1), true);
        assert_eq!(graph.bfs(2, 2), false);
        assert_eq!(graph.bfs(4, 4), false);
        assert_eq!(graph.bfs(3, 3), true);
    }
}
