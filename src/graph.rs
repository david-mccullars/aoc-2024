use hashbrown::hash_map::Entry::{Occupied, Vacant};
use hashbrown::{HashMap, HashSet};
use num::{Bounded, Num, Saturating, Zero};
use petgraph::dot::Dot;
use petgraph::graph::IndexType;
use petgraph::graph::NodeIndex;
use petgraph::visit::GraphProp;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeIndexable;
use petgraph::EdgeType;
use petgraph::Graph;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn write_graph<G>(graph: G, filename: &str)
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: std::fmt::Display,
    G::NodeWeight: std::fmt::Display,
{
    // Convert the graph into DOT format
    // let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot = Dot::with_config(graph, &[]);
    let dot_string = format!("{}", dot);

    // Write the DOT string to a temporary file
    let dot_filename = "temp.dot";
    let mut file = File::create(dot_filename).expect("Unable to create file");
    file.write_all(dot_string.as_bytes())
        .expect("Unable to write data");

    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    // Use the `dot` command-line tool (part of the Graphviz software package) to convert the DOT file to SVG
    let output = Command::new("dot")
        .arg(format!("-T{}", ext).as_str())
        .arg(dot_filename)
        .arg("-o")
        .arg(filename)
        .output()
        .expect("Failed to execute command");

    // Check the output of the command
    if !output.status.success() {
        eprintln!(
            "dot command failed with output:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Delete the temporary DOT file
    std::fs::remove_file(dot_filename).expect("Failed to remove temporary file");
}

pub fn floyd_warshall<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
) -> HashMap<(NodeIndex<Ix>, NodeIndex<Ix>), E>
where
    E: Num + Bounded + Zero + Saturating + PartialOrd + Copy,
    Ty: EdgeType,
    Ix: IndexType,
{
    let mut distances = HashMap::new();
    let nodes: Vec<_> = graph.node_identifiers().collect();

    // Initialize distances
    for node in &nodes {
        for target in &nodes {
            if node == target {
                distances.insert((*node, *target), E::zero());
            } else if let Some(edge) = graph.find_edge(*node, *target) {
                distances.insert((*node, *target), graph[edge]);
            } else {
                distances.insert((*node, *target), E::max_value());
            }
        }
    }

    // Floyd-Warshall algorithm
    for k in &nodes {
        for i in &nodes {
            for j in &nodes {
                let ikj = distances[&(*i, *k)].saturating_add(distances[&(*k, *j)]);
                if ikj < distances[&(*i, *j)] {
                    distances.insert((*i, *j), ikj);
                }
            }
        }
    }

    distances
}

struct LeastCost<N, K> {
    node: N,
    cost: K,
}

impl<N, K: PartialEq> PartialEq for LeastCost<N, K> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N, K: PartialEq> Eq for LeastCost<N, K> {}

impl<N, K: Ord> PartialOrd for LeastCost<N, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, K: Ord> Ord for LeastCost<N, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Special-purpose alternative version of dijkstra which attempts
// to find ALL shortest paths from start to stop. It returns the
// unique set of all nodes visited on at least one of the shortest
// paths, along with the shortest cost. If no path can be found
// then it returns None
pub fn dijkstra_multi<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut stop: FS,
) -> Option<(HashSet<N>, C)>
where
    N: Eq + std::hash::Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(LeastCost {
        node: start.clone(),
        cost: C::zero(),
    });

    let mut parents = HashMap::new();
    parents.insert(start.clone(), (vec![], C::zero()));

    let mut target_reached = None;

    while let Some(LeastCost { node, cost }) = to_see.pop() {
        let successors = {
            if stop(&node) {
                target_reached = Some(node.clone());
                break;
            }
            successors(&node)
        };
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            match parents.entry(successor.clone()) {
                Vacant(e) => {
                    e.insert((vec![node.clone()], new_cost));
                }
                Occupied(mut e) => {
                    let old_cost = e.get().1;
                    if new_cost < old_cost {
                        e.insert((vec![node.clone()], new_cost));
                    } else if new_cost == old_cost {
                        // THIS IS WHERE WE DEVIATE FROM DIJKSTRA!
                        // Normally we'd skip this case, but here we're going
                        // to keep track of all parents with the shortest cost
                        e.get_mut().0.push(node.clone());
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(LeastCost {
                node: successor,
                cost: new_cost,
            });
        }
    }

    if let Some(end) = target_reached {
        let mut visited_on_shortest = HashSet::new();
        let mut stack = vec![end.clone()];
        while let Some(n) = stack.pop() {
            visited_on_shortest.insert(n.clone());
            for parent in &parents.get(&n).unwrap().0 {
                if !visited_on_shortest.contains(parent) {
                    stack.push(parent.clone());
                }
            }
        }
        Some((visited_on_shortest, parents.get(&end).unwrap().1))
    } else {
        None
    }
}
