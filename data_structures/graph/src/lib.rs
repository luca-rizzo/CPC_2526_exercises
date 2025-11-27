pub struct Dag {
    adj_lists: Vec<Vec<(usize, u32)>>,
}

impl Dag {
    pub fn new(num_nodes: usize) -> Self {
        if num_nodes == 0 {
            panic!("Num nodes should be > 0");
        }
        Self {
            adj_lists: vec![Vec::new(); num_nodes],
        }
    }

    fn from_edges_weighted(num_nodes: usize, edges: &[(usize, usize, u32)]) -> Self {
        let mut inst = Self::new(num_nodes);
        for &(s, d, w) in edges {
            inst.add_edge(s, d, w);
        }
        inst
    }

    pub fn add_edge(&mut self, s: usize, e: usize, w: u32) {
        self.adj_lists[s].push((e, w));
    }

    fn topological_sort(&self) -> Vec<usize> {
        let mut queue = std::collections::VecDeque::new();
        let mut order = Vec::new();
        let mut visited = vec![false; self.adj_lists.len()];
        queue.push_back(0usize);
        while let Some(node_id) = queue.pop_front() {
            if visited[node_id] == false {
                for &(neighbor, _) in &self.adj_lists[node_id] {
                    if visited[node_id] == false {
                        queue.push_back(neighbor);
                    }
                }
                visited[node_id] = true;
                order.push(node_id);
            }
        }
        order
    }
}

pub fn best_paths_from_source<F>(
    dag: &Dag,
    source: usize,
    better: F,
) -> Vec<Option<(usize, u32)>>
where
    F: Fn(u32, u32) -> bool + Copy,
{
    let mut path = vec![None; dag.adj_lists.len()];
    path[source] = Some((source, 0)); // parent=source per coerenza
    let topological_sort = dag.topological_sort();

    for node in topological_sort {
        for &(neighbor, weight) in &dag.adj_lists[node] {
            if let Some((_, visited_cost)) = path[node] {
                let new_cost = visited_cost + weight;
                match path[neighbor] {
                    None => {
                        path[neighbor] = Some((node, new_cost));
                    }
                    Some((_, old_cost)) if better(new_cost, old_cost) => {
                        path[neighbor] = Some((node, new_cost));
                    }
                    _ => {}
                }
            }
        }
    }
    path
}

pub fn minimum_paths_cost(dag: &Dag, source: usize)
                          -> Vec<Option<(usize, u32)>>
{
    best_paths_from_source(dag, source, |new, old| new < old)
}

pub fn maximum_paths_beneficial(dag: &Dag, source: usize)
                                -> Vec<Option<(usize, u32)>>
{
    best_paths_from_source(dag, source, |new, old| new > old)
}

pub fn extract_path(
    path_info: &[Option<(usize, u32)>],
    source: usize,
    dest: usize,
) -> Option<(Vec<usize>, u32)> {
    let (_, total_cost) = path_info[dest]?;

    let mut path = Vec::new();
    let mut current = dest;

    while current != source {
        let (parent, _) = path_info[current]?;
        path.push(current);
        current = parent;
    }

    path.push(source);
    path.reverse();

    Some((path, total_cost))
}

pub fn minimum_paths_cost_from_to(
    dag: &Dag,
    source: usize,
    dest: usize
) -> Option<(Vec<usize>, u32)> {
    extract_path(&minimum_paths_cost(dag, source), source, dest)
}

pub fn maximum_path_beneficial_from_to(
    dag: &Dag,
    source: usize,
    dest: usize
) -> Option<(Vec<usize>, u32)> {
    extract_path(&maximum_paths_beneficial(dag, source), source, dest)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_paths() {
        let dag = Dag::from_edges_weighted(4, &[
            (0, 1, 10),
            (1, 3, 10),
            (0, 2, 1),
            (2, 3, 1),
        ]);

        let (path, cost) = minimum_paths_cost_from_to(&dag, 0, 3).unwrap();
        assert_eq!(path, vec![0, 2, 3]);
        assert_eq!(cost, 2);
    }
}