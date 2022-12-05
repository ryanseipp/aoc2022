pub fn part_one(input: &str) -> i32 {
    return 0;
}

pub fn part_two(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(7746, part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(2604, part_two(input.as_str()));
    }
}
