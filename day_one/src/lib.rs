pub fn part_one(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();

    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut elf: Vec<i32> = Vec::new();
    for &content in contents.iter() {
        if content == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(content.parse().unwrap());
        }
    }

    return elves.iter().map(|elf| elf.iter().sum()).max().unwrap();
}

pub fn part_two(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();

    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut elf: Vec<i32> = Vec::new();
    for &content in contents.iter() {
        if content == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(content.parse().unwrap());
        }
    }

    let mut sums: Vec<i32> = elves.iter().map(|elf| elf.iter().sum()).collect();
    sums.sort_by(|a, b| b.partial_cmp(a).unwrap());

    return sums.iter().take(3).sum();
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(71924, part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(210406, part_two(input.as_str()));
    }
}
