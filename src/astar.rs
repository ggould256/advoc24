use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;


/// A* Algorithm and friends.

pub struct Pathfinder<NodeType, CostType=u32> 
    where NodeType: Display + Eq + Hash + Clone,
          CostType: Add + Ord + Eq + num::Bounded {
    
    // The public-facing interface.
    neighbors_fn: Box<dyn Fn(NodeType) -> HashMap<NodeType, CostType>>,
    heuristic_fn: Box<dyn Fn(NodeType) -> CostType>,

    // Cached data.

    /// Memoized values of heuristic_fn
    heuristics_cache: HashMap<NodeType, CostType>,

    /// Memoized values of neighbors_fn
    transitions_cache: HashMap<(NodeType, NodeType), CostType>,

    /// The "backtrack" map:
    /// {(a, b) -> c} => on a path to b, a was preceded by c.
    path_predecessors: HashMap<(NodeType, NodeType), NodeType>,

    /// The "G score" map:
    /// {(a, b) -> s} => the lowest-cost known path from a to b had score s.
    best_known_path_scores: HashMap<(NodeType, NodeType), CostType>,

}



impl<NodeType, CostType> Pathfinder<NodeType, CostType> 
where NodeType: Display + Eq + Hash + Clone,
      CostType: Add + Ord + Eq + num::Bounded {
    pub fn new(
            neighbors_fn: Box<dyn Fn(NodeType) -> HashMap<NodeType, CostType>>,
            heuristic_fn: Box<dyn Fn(NodeType) -> CostType>) -> Pathfinder<NodeType, CostType> {
        Pathfinder{neighbors_fn, heuristic_fn,
                   nodes_cache: HashMap::new(), transitions_cache: HashMap::new(),}
    }

    fn find_path_internal(&mut self, from: &HashSet<NodeType>, to: HashSet<NodeType>) {
        let mut open_set = from.clone();

    }

    pub fn find_path(&self, from: NodeType, to: NodeType) -> Vec<NodeType> {
        Vec::new()
    }
}


/*

function reconstruct_path(cameFrom, current)
    total_path := {current}
    while current in cameFrom.Keys:
        current := cameFrom[current]
        total_path.prepend(current)
    return total_path

// A* finds a path from start to goal.
// h is the heuristic function. h(n) estimates the cost to reach goal from node n.
function A_Star(start, goal, h)
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    openSet := {start}

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
    // to n currently known.
    cameFrom := an empty map

    // For node n, gScore[n] is the currently known cost of the cheapest path from start to n.
    gScore := map with default value of Infinity
    gScore[start] := 0

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    fScore := map with default value of Infinity
    fScore[start] := h(start)

    while openSet is not empty
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        current := the node in openSet having the lowest fScore[] value
        if current = goal
            return reconstruct_path(cameFrom, current)

        openSet.Remove(current)
        for each neighbor of current
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            tentative_gScore := gScore[current] + d(current, neighbor)
            if tentative_gScore < gScore[neighbor]
                // This path to neighbor is better than any previous one. Record it!
                cameFrom[neighbor] := current
                gScore[neighbor] := tentative_gScore
                fScore[neighbor] := tentative_gScore + h(neighbor)
                if neighbor not in openSet
                    openSet.add(neighbor)

    // Open set is empty but goal was never reached
    return failure


*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trivial() {
        let dut = Pathfinder::new(
            Box::new(move |this: u8| HashMap::from([(this+1, 1)])),
            Box::new(move |this: u8| 5 - this));
        let path = dut.find_path(1, 5);
        assert_eq!(path, [1, 2, 3, 4, 5].to_vec());
    }

}