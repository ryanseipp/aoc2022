use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

#[derive(Debug)]
struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing direction")
    }
}

impl From<ParseIntError> for ParseDirectionError {
    fn from(_: ParseIntError) -> Self {
        ParseDirectionError
    }
}

impl Error for ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let count = split[1].parse()?;
        return match split[0] {
            "U" => Ok(Direction::Up(count)),
            "D" => Ok(Direction::Down(count)),
            "R" => Ok(Direction::Right(count)),
            "L" => Ok(Direction::Left(count)),
            _ => Err(ParseDirectionError),
        };
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        return (self.y - other.y).abs().max((self.x - other.x).abs());
    }
}

#[derive(Debug)]
struct Rope {
    pub head: Position,
    pub tail: Vec<Position>,
}

impl Rope {
    pub fn new(tail_size: usize) -> Self {
        Rope {
            head: Position::default(),
            tail: vec![Position::default(); tail_size],
        }
    }

    pub fn move_direction(&mut self, direction: Direction) -> Vec<Position> {
        let mut position = Vec::default();
        match direction {
            Direction::Up(count) => (0..count).for_each(|_| {
                self.head.y += 1;
                for i in 0..self.tail.len() {
                    self.move_tail(i);
                }
                position.push(self.tail.last().unwrap().clone());
            }),
            Direction::Down(count) => (0..count).for_each(|_| {
                self.head.y -= 1;
                for i in 0..self.tail.len() {
                    self.move_tail(i);
                }
                position.push(self.tail.last().unwrap().clone());
            }),
            Direction::Right(count) => (0..count).for_each(|_| {
                self.head.x += 1;
                for i in 0..self.tail.len() {
                    self.move_tail(i);
                }
                position.push(self.tail.last().unwrap().clone());
            }),
            Direction::Left(count) => (0..count).for_each(|_| {
                self.head.x -= 1;
                for i in 0..self.tail.len() {
                    self.move_tail(i);
                }
                position.push(self.tail.last().unwrap().clone());
            }),
        }

        return position;
    }

    fn move_tail(&mut self, idx: usize) {
        let prev_tail = if idx == 0 {
            &self.head
        } else {
            &self.tail[idx - 1]
        };

        if self.tail[idx].distance(prev_tail) <= 1 {
            return;
        }

        let move_x = (prev_tail.x - self.tail[idx].x).clamp(-1, 1);
        let move_y = (prev_tail.y - self.tail[idx].y).clamp(-1, 1);

        if move_x.abs() == move_y.abs() {
            self.tail[idx].x += move_x;
            self.tail[idx].y += move_y;
        } else if move_x.abs() > move_y.abs() {
            self.tail[idx].x += move_x;
        } else {
            self.tail[idx].y += move_y;
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut rope = Rope::new(1);
    let mut tail_positions = vec![Position::default()];

    input
        .lines()
        .map(|i| Direction::from_str(i).unwrap())
        .for_each(|direction| tail_positions.append(&mut rope.move_direction(direction)));

    tail_positions.sort();
    tail_positions.dedup();

    return tail_positions.len();
}

pub fn part_two(input: &str) -> usize {
    let mut rope = Rope::new(9);
    let mut tail_positions = vec![Position::default()];

    input
        .lines()
        .map(|i| Direction::from_str(i).unwrap())
        .for_each(|direction| tail_positions.append(&mut rope.move_direction(direction)));

    tail_positions.sort();
    tail_positions.dedup();

    return tail_positions.len();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn part_one_computes_correct_result() {
        assert_eq!(6023, part_one(&get_input()));
    }

    #[test]
    fn part_two_computes_correct_result() {
        assert_eq!(2533, part_two(&get_input()));
    }

    #[test]
    fn part_one_computes_example() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(13, part_one(input));
    }

    #[test]
    fn part_two_computes_example() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";
        assert_eq!(36, part_two(input));
    }
}
