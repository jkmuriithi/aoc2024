//! Advent of Code Day 4

use std::{collections::HashSet, error::Error, ops::Div, time::Instant};

/// Location in the word search matrix
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RowCol(usize, usize);

/// Direction that the word is written in, starting from the first character
/// going to the last
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Direction {
    fn all() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [North, South, West, East, Northeast, Northwest, Southeast, Southwest].into_iter()
    }

    fn neighbor<T>(&self, matrix: &[Vec<T>], RowCol(row, col): RowCol) -> Option<RowCol> {
        match self {
            Direction::North => (row > 0).then_some(RowCol(row - 1, col)),
            Direction::South => (row < matrix.len() - 1).then_some(RowCol(row + 1, col)),
            Direction::East => (col < matrix[row].len() - 1).then_some(RowCol(row, col + 1)),
            Direction::West => (col > 0).then_some(RowCol(row, col - 1)),
            Direction::Northeast => {
                (row > 0 && col < matrix[row].len() - 1).then_some(RowCol(row - 1, col + 1))
            }
            Direction::Northwest => (row > 0 && col > 0).then_some(RowCol(row - 1, col - 1)),
            Direction::Southeast => (row < matrix.len() - 1 && col < matrix[row].len() - 1)
                .then_some(RowCol(row + 1, col + 1)),
            Direction::Southwest => {
                (row < matrix.len() - 1 && col > 0).then_some(RowCol(row + 1, col - 1))
            }
        }
    }
}

/// Uniquely identifies a search hit in the word search matrix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Hit {
    start: RowCol,
    direction: Direction,
    len: usize,
}

impl Hit {
    /// Calculates the [Hit] objects which would form an "X" shape in the word
    /// search matrix with this hit   
    ///
    /// Note: The returned hits always start at valid locations, given the word
    /// search matrix is square
    fn x_neighbors(&self) -> Vec<Hit> {
        assert!(self.len >= 2, "words of length 0-1 cannot form an X shape");

        match self.direction {
            Direction::Northeast => {
                vec![
                    Hit {
                        start: RowCol(self.start.0, self.start.1 + (self.len - 1)),
                        direction: Direction::Northwest,
                        len: self.len,
                    },
                    Hit {
                        start: RowCol(self.start.0 - (self.len - 1), self.start.1),
                        direction: Direction::Southeast,
                        len: self.len,
                    },
                ]
            }
            Direction::Northwest => {
                vec![
                    Hit {
                        start: RowCol(self.start.0, self.start.1 - (self.len - 1)),
                        direction: Direction::Northeast,
                        len: self.len,
                    },
                    Hit {
                        start: RowCol(self.start.0 - (self.len - 1), self.start.1),
                        direction: Direction::Southwest,
                        len: self.len,
                    },
                ]
            }
            Direction::Southeast => {
                vec![
                    Hit {
                        start: RowCol(self.start.0 + (self.len - 1), self.start.1),
                        direction: Direction::Northeast,
                        len: self.len,
                    },
                    Hit {
                        start: RowCol(self.start.0, self.start.1 + (self.len - 1)),
                        direction: Direction::Southwest,
                        len: self.len,
                    },
                ]
            }
            Direction::Southwest => {
                vec![
                    Hit {
                        start: RowCol(self.start.0 + (self.len - 1), self.start.1),
                        direction: Direction::Northwest,
                        len: self.len,
                    },
                    Hit {
                        start: RowCol(self.start.0, self.start.1 - (self.len - 1)),
                        direction: Direction::Southeast,
                        len: self.len,
                    },
                ]
            }
            _ => vec![],
        }
    }
}

fn search_rec(
    matrix: &Vec<Vec<char>>,
    word: &[char],
    direction: Direction,
    start: RowCol,
    curr: RowCol,
    idx: usize,
) -> Option<Hit> {
    if matrix[curr.0][curr.1] != word[idx] {
        None
    } else if idx == word.len() - 1 {
        Some(Hit { start, direction, len: word.len() })
    } else {
        direction
            .neighbor(matrix, curr)
            .and_then(|next| search_rec(matrix, word, direction, start, next, idx + 1))
    }
}

fn search_word(matrix: &Vec<Vec<char>>, word: &[char]) -> HashSet<Hit> {
    (0..matrix.len())
        .flat_map(|row| (0..matrix[row].len()).map(move |col| (row, col)))
        .filter(|(row, col)| matrix[*row][*col] == word[0])
        .flat_map(|(row, col)| {
            Direction::all().filter_map(move |dir| {
                search_rec(matrix, word, dir, RowCol(row, col), RowCol(row, col), 0)
            })
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let matrix: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.map(|s| s.chars().collect::<Vec<_>>()))
        .collect::<Result<_, _>>()?;

    let xmas: Vec<_> = "XMAS".chars().collect();
    let xmas_hits = search_word(&matrix, &xmas);
    println!("Number of times XMAS appears: {}", xmas_hits.len());

    let mas: Vec<_> = "MAS".chars().collect();
    let mas_hits = search_word(&matrix, &mas);
    let x_count = mas_hits
        .iter()
        .filter(|hit| hit.x_neighbors().iter().any(|hit| mas_hits.contains(hit)))
        .count()
        .div(2);
    println!("Number of times X-MAS appears: {}", x_count);

    println!("Elapsed time: {}ms", start.elapsed().as_millis());

    Ok(())
}
