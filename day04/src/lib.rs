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
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(821, part_two(input.as_str()));
    }
}
