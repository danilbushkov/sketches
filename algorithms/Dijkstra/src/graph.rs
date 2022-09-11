
use std::collections::HashMap;
use std::collections::HashSet;


pub struct Graph {
    vertices: HashMap<usize, HashMap<usize, usize>>,
}


impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    pub fn add_vertice(&mut self, vertice: usize, edges: HashMap<usize, usize>) {
        self.vertices.insert(vertice, edges);
    }

    pub fn shortcut(&self, start: usize, target: usize) -> Vec<usize> {
        let mut costs: HashMap<usize, usize> = HashMap::new();
        let mut parents: HashMap<usize, usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();

        let mut vertice = Some(start);
        let mut cost = 0;
        while let Some(parent) = vertice.take() {
            if parent == target {
                if parents.contains_key(&parent) {
                    vertice = Some(target);
                    break;
                }
            }

            if let Some(n) = self.vertices.get(&parent) {
                for (v, c) in n {
                    if !visited.contains(v) {
                        if let Some(current_cost) = costs.get(&v) {
                            if cost + c < *current_cost {
                                costs.insert(*v, cost + c);
                                parents.insert(*v, parent);
                            } 
                        } else {
                            costs.insert(*v, cost + c);
                            parents.insert(*v, parent);
                        }
                    }
                    
                }
            };
            visited.insert(parent);

            cost = 0;
            let mut lowest = usize::MAX;
            for (v, c) in &costs {
                if *c < lowest && !visited.contains(&v) {
                    lowest = *c;
                    vertice = Some(*v);
                    cost = *c;
                }
            }
        }
        if let Some(v) = vertice {
            let mut path = vec![v];
            let mut k = v;
            while let Some(p) = parents.get(&k) {
                path.insert(0, *p);
                k = *p;
            }

            return path;
        }

        vec![]
    }

   

}