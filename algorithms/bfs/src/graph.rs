
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

pub struct Graph {
    vertices: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    pub fn add_vertice(&mut self, number: usize, edges: HashSet<usize>) {
        self.vertices.insert(number, edges);
    }

    pub fn bfs(&self, start: usize, target: usize) -> bool {
        let mut queue = LinkedList::<usize>::new();
        let mut labeled_vertices = HashSet::<usize>::new();
        if let Some(set) = self.vertices.get(&start) {
            queue.extend(set.iter());
        }
        labeled_vertices.insert(start);
        while !queue.is_empty() {
            if let Some(item) = queue.pop_front() {
                if item == target {
                    return true;
                } 
                if !labeled_vertices.contains(&item) {
                    if let Some(set) = self.vertices.get(&item) {
                        queue.extend(set.iter());
                    }
                }
                labeled_vertices.insert(item);
            }
        }


        false
    }



}