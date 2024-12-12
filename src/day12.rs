use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::parsing::read_lines;

type Color = char;
type Coords = (usize, usize);
type Map = Vec<Vec<Color>>;
type Adjacency = HashSet<(Coords, Coords)>;

fn make_map(input: &Vec<String>) -> Map {
    let mut result = Map::new();
    for line in input {
        result.push(line.chars().collect());
    }
    result
}

fn at(map: Map, xy: Coords) {
    map[y][x]
}

fn map_to_string(map: Map) -> String {
    let h = map.len();
    let w = map[0].len();
    let mut result: String = String::new();
    for (y, row) in map.iter().enumerate() {
        let row_string: String = row.iter().collect();
        result += &row_string;
        result += "\n";
    }
    result
}

enum AdjacencyOpt {
    SameColor,
    DifferentColor,
}

fn make_adjacency(map: &Map, style: AdjacencyOpt) -> Adjacency {
    let mut raw_adjacency: Adjacency = Adjacency::new();
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            if x > 0 {
                raw_adjacency.insert(((x, y), (x - 1, y)));
            }
            if y > 0 {
                raw_adjacency.insert(((x, y), (x, y - 1)));
            }
            if x < w - 1 {
                raw_adjacency.insert(((x, y), (x + 1, y)));
            }
            if y < h - 1 {
                raw_adjacency.insert(((x, y), (x, y + 1)));
            }
        }
    }
    let raw_adjacency = raw_adjacency; // Drop mutability.
    let mut result = HashSet::new();
    for from_to in raw_adjacency {
        let ((from_x, from_y), (to_x, to_y)) = from_to;
        let from_color = map[from_y][from_x];
        let to_color = map[to_y][to_x];
        if match style {
            AdjacencyOpt::SameColor => from_color == to_color,
            AdjacencyOpt::DifferentColor => from_color != to_color,
        } {
            result.insert(from_to);
        }
    }
    result
}

fn make_successors(adjacency: &Adjacency) -> HashMap<Coords, HashSet<Coords>> {
    let mut result: HashMap<Coords, HashSet<Coords>> = HashMap::new();
    for (from, to) in adjacency {
        result.entry(*from).or_default().insert(*to);
    }
    result
}

fn all_of(map: &Map, color: Color) -> HashSet<Coords> {
    let mut result = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, found_color) in row.iter().enumerate() {
            if *found_color == color {
                result.insert((x, y));
            }
        }
    }
    result
}

fn reachable(map: &Map, start: &Coords) -> HashSet<Coords> {
    let adjacency = make_adjacency(map, AdjacencyOpt::SameColor);
    let successors = make_successors(&adjacency);
    // Simple breadth-first search of the adjacency graph.
    let mut worklist: Vec<Coords> = vec![*start];
    let mut visited: HashSet<Coords> = HashSet::new();
    while let Some(from) = worklist.pop() {
        if successors.contains_key(&from) {
            for to in successors[&from].iter() {
                worklist.push(*to);
            }
        }
        visited.insert(from);
    }
    visited
}

pub fn day12(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let map = make_map(&lines);
    0
}

pub fn day12b(source: Option<String>) -> i64 {
    day12(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example2() {
        assert_eq!(day12(Some("data/day12_example1.txt".to_string())), 140);
        assert_eq!(day12(Some("data/day12_example2.txt".to_string())), 772);
        assert_eq!(day12(Some("data/day12_example3.txt".to_string())), 1930);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day12(Some("inputs/day12_test.txt".to_string())), 489);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day12(Some("data/day12_example1.txt".to_string())), 140);
        assert_eq!(day12(Some("data/day12_example2.txt".to_string())), 772);
        assert_eq!(day12(Some("data/day12_example3.txt".to_string())), 1930);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day12b(Some("inputs/day12_test.txt".to_string())), 1086);
    }
}
