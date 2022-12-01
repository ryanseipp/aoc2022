use std::fs;

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

fn part_one() -> i32 {
    let raw_input = fs::read_to_string("input.txt").expect("File input.txt should exist");
    let contents: Vec<&str> = raw_input.split('\n').collect();

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

fn part_two() -> i32 {
    let raw_input = fs::read_to_string("input.txt").expect("File input.txt should exist");
    let contents: Vec<&str> = raw_input.split('\n').collect();

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
    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        assert_eq!(71924, part_one());
    }

    #[test]
    fn part_two_returns_correct_result() {
        assert_eq!(210406, part_two());
    }
}
