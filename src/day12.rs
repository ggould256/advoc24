use std::collections::{HashMap, HashSet};

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

fn at(map: &Map, (x, y): Coords) -> Color {
    map[y][x]
}

#[allow(dead_code)]
fn map_to_string(map: &Map) -> String {
    let mut result: String = String::new();
    for row in map.iter() {
        let row_string: String = row.iter().collect();
        result += &row_string;
        result += "\n";
    }
    result
}

#[allow(dead_code)]
enum AdjacencyOpt {
    All,
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
            AdjacencyOpt::All => true,
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

#[allow(dead_code)]
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

fn reachable(start: &Coords, adjacency: &Adjacency) -> HashSet<Coords> {
    let successors = make_successors(&adjacency);
    // Simple breadth-first search of the adjacency graph.
    let mut worklist: Vec<Coords> = vec![*start];
    let mut visited: HashSet<Coords> = HashSet::new();
    while let Some(from) = worklist.pop() {
        if visited.contains(&from) {
            continue;
        }
        if successors.contains_key(&from) {
            for to in successors[&from].iter() {
                worklist.push(*to);
            }
        }
        visited.insert(from);
    }
    visited
}

fn make_region(map: &Map, start: &Coords) -> HashSet<Coords> {
    reachable(start, &make_adjacency(map, AdjacencyOpt::SameColor))
}

fn make_regions(map: &Map) -> Vec<HashSet<Coords>> {
    let mut consumed: HashSet<Coords> = HashSet::new();
    let mut result = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !consumed.contains(&(x, y)) {
                println!(
                    "Creating new region of {} starting from {:?} ...",
                    at(map, (x, y)),
                    &(x, y)
                );
                let region = make_region(map, &(x, y));
                consumed.extend(&region);
                println!("  ...of size {}", region.len());
                result.push(region);
            }
        }
    }
    result
}

fn score_region(map: &Map, region: &HashSet<Coords>) -> i64 {
    let mut score = 0;
    let size = region.len();
    let same_color_neighbors = make_successors(&make_adjacency(map, AdjacencyOpt::SameColor));
    for point in region {
        let neighbor_count = match same_color_neighbors.get(point) {
            Some(neighbors) => neighbors.len(),
            None => 0,
        };
        let fence_count = 4 - neighbor_count;
        score += fence_count * size;
    }
    score.try_into().unwrap()
}

pub fn day12(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let map = make_map(&lines);
    let regions = make_regions(&map);
    regions.iter().map(|r| score_region(&map, r)).sum()
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
