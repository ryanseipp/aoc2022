use std::cmp::Ordering;

pub fn part_one(input: &str) -> i32 {
    let assignments: Vec<&str> = input.split('\n').collect();
    let mut overlap: i32 = 0;

    for assignment in assignments {
        if assignment.len() == 0 {
            continue;
        }

        let pairs: Vec<_> = assignment.split(',').collect();
        let first_elf: Vec<i32> = pairs
            .first()
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second_elf: Vec<i32> = pairs
            .iter()
            .nth(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();

        if (first_elf[0] <= second_elf[0] && first_elf[1] >= second_elf[1])
            || (second_elf[0] <= first_elf[0] && second_elf[1] >= first_elf[1])
        {
            overlap += 1;
        }
    }

    return overlap;
}

pub fn part_one_by_char(input: &str) -> Result<usize, String> {
    return determine_by_char(input, |ranges| {
        let first = ranges[0].cmp(&ranges[2]);
        let second = ranges[1].cmp(&ranges[3]);
        return first == Ordering::Equal || second == Ordering::Equal || first != second;
    });
}

pub fn part_two_by_char(input: &str) -> Result<usize, String> {
    return determine_by_char(input, |ranges| {
        let first_a = ranges[0].cmp(&ranges[2]);
        let first_b = ranges[0].cmp(&ranges[3]);
        let second_a = ranges[1].cmp(&ranges[2]);
        let second_b = ranges[1].cmp(&ranges[3]);
        return first_a == Ordering::Equal
            || first_b == Ordering::Equal
            || second_a == Ordering::Equal
            || second_b == Ordering::Equal
            || first_a != second_a
            || first_b != second_b
            || first_a != first_b
            || second_a != second_b;
    });
}

fn determine_by_char<F>(input: &str, is_overlap: F) -> Result<usize, String>
where
    F: Fn(&[usize; 4]) -> bool,
{
    let mut overlaps: usize = 0;
    let mut indicies: [usize; 8] = [0; 8];
    let mut ranges: [usize; 4] = [0; 4];
    let mut idx: usize = 0;

    for (i, c) in input.chars().enumerate() {
        if idx == 0 {
            indicies[idx] = i;
            idx += 1;
        } else if c == '-' || c == ',' {
            indicies[idx] = i;
            indicies[idx + 1] = i + 1;
            idx += 2;
        } else if c == '\n' {
            indicies[idx] = i;

            for j in (0..indicies.len()).step_by(2) {
                ranges[j / 2] = input
                    .get(indicies[j]..indicies[j + 1])
                    .ok_or("Could not retrieve range")?
                    .parse()
                    .map_err(|_| "Could not parse number")?;
            }

            if is_overlap(&ranges) {
                overlaps += 1;
            }

            idx = 0;
        }
    }

    return Ok(overlaps);
}

pub fn part_two(input: &str) -> i32 {
    let assignments: Vec<&str> = input.split('\n').collect();
    let mut overlap: i32 = 0;

    for assignment in assignments {
        if assignment.len() == 0 {
            continue;
        }

        let pairs: Vec<_> = assignment.split(',').collect();
        let first_elf: Vec<i32> = pairs
            .first()
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second_elf: Vec<i32> = pairs
            .iter()
            .nth(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();

        if (first_elf[0] <= second_elf[0] && first_elf[1] >= second_elf[0])
            || (first_elf[0] <= second_elf[1] && first_elf[1] >= second_elf[1])
            || ((second_elf[0] <= first_elf[0] && second_elf[1] >= first_elf[0])
                || second_elf[0] <= first_elf[1] && second_elf[1] >= first_elf[1])
        {
            overlap += 1;
        }
    }

    return overlap;
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(511, part_one(input.as_str()));
    }

    #[test]
    fn part_one_by_char_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(Ok(511), part_one_by_char(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(821, part_two(input.as_str()));
    }

    #[test]
    fn part_two_by_char_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(Ok(821), part_two_by_char(input.as_str()));
    }
}
