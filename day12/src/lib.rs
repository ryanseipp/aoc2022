use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    position: Position,
    dist_sq: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // invert ordering so BinaryHeap becomes a min-heap
        return other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.dist_sq.cmp(&other.dist_sq));
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

pub(crate) fn distance_square(start: &Position, end: &Position) -> usize {
    return (end.row - start.row).pow(2) + (end.col - start.col).pow(2);
}

pub(crate) fn get_position_for_marker(topography: &Vec<Vec<u8>>, character: u8) -> Position {
    let pos = topography
        .iter()
        .flatten()
        .position(|c| *c == character)
        .unwrap();
    return Position::new(pos / topography[0].len(), pos % topography[0].len());
}

fn build_neighbors(position: &Position, max_row: usize, max_col: usize) -> Vec<Position> {
    let mut neighbors = Vec::default();
    if position.row != 0 {
        neighbors.push(Position::new(position.row - 1, position.col));
    }
    if position.row != max_row {
        neighbors.push(Position::new(position.row + 1, position.col));
    }
    if position.col != 0 {
        neighbors.push(Position::new(position.row, position.col - 1));
    }
    if position.col != max_col {
        neighbors.push(Position::new(position.row, position.col + 1));
    }

    return neighbors;
}

pub fn part_one(input: &str) -> Option<usize> {
    let topography: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut dist: Vec<Vec<usize>> = topography
        .iter()
        .map(|x| x.iter().map(|_| usize::MAX).collect())
        .collect();

    let start = get_position_for_marker(&topography, b'S');
    let end = get_position_for_marker(&topography, b'E');

    let mut heap = BinaryHeap::new();

    dist[start.row][start.col] = 0;
    heap.push(Node {
        cost: 0,
        position: start.clone(),
        dist_sq: distance_square(&start, &end),
    });

    while let Some(Node {
        cost,
        position,
        dist_sq: _,
    }) = heap.pop()
    {
        if position == end {
            return Some(cost);
        }

        if cost > dist[position.row][position.col] {
            continue;
        }

        for neighbor in build_neighbors(&position, topography.len(), topography[0].len()) {
            // cannot traverse when elevation > 1
            if topography[neighbor.row][neighbor.col] - topography[position.row][position.col] > 1 {
                continue;
            }

            let next = Node {
                cost: cost + 1, // validate this
                dist_sq: distance_square(&neighbor, &end),
                position: neighbor,
            };

            if next.cost < dist[next.position.row][next.position.col] {
                dist[next.position.row][next.position.col] = next.cost;
                heap.push(next);
            }
        }
    }

    return None;
}

pub fn part_two(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn determines_row_col_correctly() {
        let topography: Vec<Vec<u8>> = get_input().lines().map(|l| l.bytes().collect()).collect();
        let start = get_position_for_marker(&topography, b'S');
        assert_eq!(20, start.row);
        assert_eq!(0, start.col);
    }

    #[test]
    fn determines_correct_distance() {
        let topography: Vec<Vec<u8>> = get_input().lines().map(|l| l.bytes().collect()).collect();
        let start = get_position_for_marker(&topography, b'S');
        let end = get_position_for_marker(&topography, b'E');

        assert_eq!(43 * 43, distance_square(&start, &end));
    }
}
