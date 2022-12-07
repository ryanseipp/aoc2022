use std::collections::{BTreeMap, VecDeque};

pub fn part_one(input: &str) -> u32 {
    return check_by_map(input, 4);
}

pub fn part_one_bitwise(input: &str) -> Option<usize> {
    return bitwise_marker(input, 4);
}

pub fn part_one_bitwise_alt(input: &str) -> Option<usize> {
    return bitwise_marker_alt(input, 4);
}

pub fn part_two(input: &str) -> u32 {
    return check_by_map(input, 14);
}

pub fn part_two_bitwise(input: &str) -> Option<usize> {
    return bitwise_marker(input, 14);
}

pub fn part_two_bitwise_alt(input: &str) -> Option<usize> {
    return bitwise_marker_alt(input, 14);
}

fn check_by_map(input: &str, num: usize) -> u32 {
    let mut num_processed: u32 = 0;
    let mut sequence: VecDeque<char> = VecDeque::new();
    let mut sequence_set: BTreeMap<char, u8> = BTreeMap::new();

    for char in input.chars() {
        if sequence.len() == num {
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

fn bitwise_marker(input: &str, num: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut filter = 0u32;

    bytes
        .iter()
        .take(num - 1)
        .for_each(|c| filter ^= 1 << (c - b'a'));

    return bytes
        .windows(num)
        .position(|set| {
            filter ^= 1 << (set[set.len() - 1] - b'a');
            let result = filter.count_ones() == num as u32;
            filter ^= 1 << (set[0] - b'a');
            return result;
        })
        .map(|x| x + num);
}

fn bitwise_marker_alt(input: &str, num: usize) -> Option<usize> {
    return input
        .as_bytes()
        .windows(num)
        .position(|set| {
            let mut data: u32 = 0;
            for &c in set {
                let prev = data;
                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            return true;
        })
        .map(|x| x + num);
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn part_one_returns_correct_result() {
        assert_eq!(1723, part_one(&get_input()));
    }

    #[test]
    fn part_one_bitwise_returns_correct_result() {
        assert_eq!(Some(1723), part_one_bitwise(&get_input()));
    }

    #[test]
    fn part_one_bitwise_alt_returns_correct_result() {
        assert_eq!(Some(1723), part_one_bitwise_alt(&get_input()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        assert_eq!(3708, part_two(&get_input()));
    }

    #[test]
    fn part_two_bitwise_returns_correct_result() {
        assert_eq!(Some(3708), part_two_bitwise(&get_input()));
    }

    #[test]
    fn part_two_bitwise_alt_returns_correct_result() {
        assert_eq!(Some(3708), part_two_bitwise_alt(&get_input()));
    }
}
