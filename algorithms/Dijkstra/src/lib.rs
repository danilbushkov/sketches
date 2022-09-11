pub mod graph;





#[cfg(test)]
mod tests {

    use crate::graph::Graph;
    use std::collections::HashMap;

    #[test]
    fn test() {

        let mut graph = Graph::new();

        graph.add_vertice(1, HashMap::from([(2, 1), (4, 0), (5, 0)]));
        graph.add_vertice(2, HashMap::from([(3, 3)]));
        graph.add_vertice(3, HashMap::from([(1, 1), (8, 0), (9, 6)]));
        graph.add_vertice(4, HashMap::from([(6, 3), (9, 5)]));
        graph.add_vertice(5, HashMap::from([(5, 0)]));
        graph.add_vertice(6, HashMap::from([(1, 1)]));
        graph.add_vertice(7, HashMap::from([]));
        graph.add_vertice(8, HashMap::from([(5, 3), (7, 2)]));
        graph.add_vertice(9, HashMap::from([(10, 1)]));
        graph.add_vertice(10, HashMap::from([(7, 3)]));

        
        assert_eq!(graph.shortcut(1, 5), [1, 5]);
        assert_eq!(graph.shortcut(5, 8), []);
        assert_eq!(graph.shortcut(1, 6), [1, 4, 6]);
        assert_eq!(graph.shortcut(1, 7), [1, 2, 3, 8, 7]);
        assert_eq!(graph.shortcut(1, 10), [1, 4, 9, 10]);
        assert_eq!(graph.shortcut(1, 1), []);
        assert_eq!(graph.shortcut(3, 5), [3, 1, 5]);
    }
}
