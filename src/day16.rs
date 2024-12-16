use std::collections::HashSet;

use crate::parsing::read_lines;
use crate::grid_board::{self, Board, Direction, Xy};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BoardContent {
    Wall, Empty
}

impl TryFrom<char> for BoardContent {
    type Error = String;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(BoardContent::Wall),
            '.' | 'S' | 'E' => Ok(BoardContent::Empty),
            _ => Err(format!("Illegal character in board string: `{}`", c))
        }
    }
}

impl ToString for BoardContent {
    fn to_string(&self) -> String {
        match self {
            BoardContent::Wall => "#",
            BoardContent::Empty => ".",
        }.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    position: Xy,
    facing: Direction
}

#[derive(Debug, Clone)]
struct Puzzle {
    board: Board<BoardContent>,
    start: Xy,
    end: Xy,
    start_direction: Direction
}

impl Puzzle {
    pub fn from_strings(input: Vec<String>) -> Puzzle {
        let board = Board::from_strings(&input);
        let mut start: Xy = Xy::new(-1, -1);
        let mut end: Xy = Xy::new(-1, -1);
        let start_direction = Direction::East;
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == 'S' { start = Xy::new(x as grid_board::Scalar, y as grid_board::Scalar) }
                if c == 'E' { end = Xy::new(x as grid_board::Scalar, y as grid_board::Scalar) }
            }
        }
        Puzzle{board, start, end, start_direction}
    }

    pub fn initial_state(&self) -> State {
        State{position: self.start, facing: self.start_direction}
    }

    pub fn final_states(&self) -> HashSet<State> {
        HashSet::from_iter(Direction::ALL.iter().map(|&d| State{position: self.start, facing: d}))
    }

    // Find a shortest path from the initial state to any final state.
    pub fn pathfind(&self) -> Vec<State> {
        self.pathfind_from_to(&self.initial_state(), &self.final_states())
    }

    // Find a shortest path from `start` to any `end`.
    pub fn pathfind_from_to(&self, start: &State, end: &HashSet<State>) -> Vec<State> {
        Vec::new()
    }
}

pub fn day16(source: Option<String>) -> i64 {
    let puzzle = Puzzle::from_strings(read_lines(source));
    puzzle.board.dimensions()[0]
}

pub fn day16b(source: Option<String>) -> i64 {
    day16(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "TODO"]
    fn test_example() {
        assert_eq!(day16(Some("data/day16_example.txt".to_string())), 10092);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day16(Some("inputs/day16_test.txt".to_string())), 1495147);
    }

    #[test]
    #[ignore = "TODO"]
    fn test_example_b() {
        assert_eq!(day16b(Some("data/day16_example.txt".to_string())), 9021);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day16b(Some("inputs/day16_test.txt".to_string())), 1524905);
    }

    // B cannot be tested.
}
