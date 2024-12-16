use std::{slice::Iter, fmt::Debug};
use nalgebra::Vector2;

pub type Scalar = i64;
pub type Xy = Vector2<Scalar>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North, East, South, West
}

impl Direction {
    pub const ALL: [Direction; 4] =
        [Direction::North, Direction::East, Direction::South, Direction::West];

    pub fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }

    pub fn to_offset(&self) -> Xy {
        match self {
            Direction::North => Xy::new(0, -1),
            Direction::East => Xy::new(1, 0),
            Direction::South => Xy::new(0, 1),
            Direction::West => Xy::new(-1, 0),
        }
    }

    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }.unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Board<BoardContent> {
    board: Vec<Vec<BoardContent>>,
}

impl<BoardContent> Board<BoardContent>
    where BoardContent: Copy + Debug + ToString,
          BoardContent: TryFrom<char, Error:Debug>
{

    pub fn height(&self) -> usize {self.board.len() }
    pub fn width(&self) -> usize { self.board[0].len() }
    pub fn dimensions(&self) -> Xy {Xy::new(self.width() as Scalar, self.height() as Scalar)}

    pub fn at(&self, xy: Xy) -> BoardContent {
        self.board[xy[1] as usize][xy[0]as usize]
    }

    pub fn set_at(&mut self, xy: Xy, c: BoardContent) {
        self.board[xy[1] as usize][xy[0]as usize] = c;
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for row in &self.board {
            result.push(String::from_iter(row.iter().map(|c| c.to_string())));
        }
        result
    }

    pub fn from_strings(strings: &Vec<String>) -> Board<BoardContent> {
        let mut result = Board{board:Vec::new()};
        for row_string in strings {
            result.board.push(row_string.chars().map(|c| BoardContent::try_from(c).unwrap()).collect());
        }
        result
    }
}
