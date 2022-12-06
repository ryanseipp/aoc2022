use std::collections::{BTreeMap, VecDeque};

pub fn part_one(input: &str) -> i32 {
    let mut num_processed: i32 = 0;
    let mut sequence: VecDeque<char> = VecDeque::new();
    let mut sequence_set: BTreeMap<char, u8> = BTreeMap::new();

    for char in input.chars() {
        if sequence.len() == 4 {
            if sequence_set.values().all(|v| *v == 1) {
                return num_processed;
            }

            if let Some(popped) = sequence.pop_front() {
                sequence_set.entry(popped).and_modify(|v| *v -= 1);
                sequence_set.retain(|_, v| *v != 0);
            };
        }

        sequence.push_back(char);
        sequence_set
            .entry(char)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        num_processed += 1;
    }

    return num_processed;
}

pub fn part_two(input: &str) -> i32 {
    let mut num_processed: i32 = 0;
    let mut sequence: VecDeque<char> = VecDeque::new();
    let mut sequence_set: BTreeMap<char, u8> = BTreeMap::new();

    for char in input.chars() {
        if sequence.len() == 14 {
            if sequence_set.values().all(|v| *v == 1) {
                return num_processed;
            }

            if let Some(popped) = sequence.pop_front() {
                sequence_set.entry(popped).and_modify(|v| *v -= 1);
                sequence_set.retain(|_, v| *v != 0);
            };
        }

        sequence.push_back(char);
        sequence_set
            .entry(char)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        num_processed += 1;
    }

    return num_processed;
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(1723, part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(3708, part_two(input.as_str()));
    }
}
