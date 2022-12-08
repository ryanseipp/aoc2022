pub fn part_one(input: &str) -> usize {
    // Row x Col
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; trees[0].len()]; trees.len()];

    // left to right
    for i in 0..trees.len() {
        let mut tallest: u32 = trees[i][0];
        visible[i][0] = true;
        for j in 0..trees[0].len() {
            if trees[i][j] > tallest {
                visible[i][j] = true;
                tallest = trees[i][j];
            }
            if trees[i][j] == 9 {
                break;
            }
        }
    }

    // right to left
    for i in 0..trees.len() {
        let mut tallest: u32 = trees[i][trees[0].len() - 1];
        visible[i][trees[0].len() - 1] = true;
        for j in (0..trees[0].len()).rev() {
            if trees[i][j] > tallest {
                visible[i][j] = true;
                tallest = trees[i][j];
            }
            if trees[i][j] == 9 {
                break;
            }
        }
    }

    // top to bottom
    for j in 0..trees[0].len() {
        let mut tallest: u32 = trees[0][j];
        visible[0][j] = true;
        for i in 0..trees.len() {
            if trees[i][j] > tallest {
                visible[i][j] = true;
                tallest = trees[i][j];
            }
            if trees[i][j] == 9 {
                break;
            }
        }
    }

    // bottom to top
    for j in 0..trees[0].len() {
        let mut tallest: u32 = trees[trees.len() - 1][j];
        visible[trees.len() - 1][j] = true;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > tallest {
                visible[i][j] = true;
                tallest = trees[i][j];
            }
            if trees[i][j] == 9 {
                break;
            }
        }
    }

    return visible.iter().fold(0, |res, row| {
        res + row.iter().fold(0, |result, col| {
            if *col {
                return result + 1;
            }
            return result;
        })
    });
}

pub fn part_two(input: &str) -> usize {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut scores: Vec<Vec<usize>> = vec![vec![0; trees[0].len()]; trees.len()];

    for row in 1..trees.len() - 1 {
        for col in 1..trees[0].len() - 1 {
            let current_tree = trees[row][col];

            // left to right
            let mut score: usize = 0;
            for i in (col + 1)..trees[0].len() {
                score += 1;
                if trees[row][i] >= current_tree {
                    break;
                }
            }
            scores[row][col] = score;

            // right to left
            score = 0;
            for i in (0..col).rev() {
                score += 1;
                if trees[row][i] >= current_tree {
                    break;
                }
            }
            scores[row][col] *= score;

            // top to bottow
            score = 0;
            for i in (row + 1)..trees.len() {
                score += 1;
                if trees[i][col] >= current_tree {
                    break;
                }
            }
            scores[row][col] *= score;

            // bottom to top
            score = 0;
            for i in (0..row).rev() {
                score += 1;
                if trees[i][col] >= current_tree {
                    break;
                }
            }
            scores[row][col] *= score;
        }
    }

    return *scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two_example() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(8, part_two(input));
    }
}
