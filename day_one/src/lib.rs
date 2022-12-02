pub fn part_one(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();
    return contents
        .split(|&i| i == "")
        .map(|elf| elf.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap();
}

pub fn part_two(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();
    let mut sums: Vec<i32> = contents
        .split(|&i| i == "")
        .map(|elf| elf.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));
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
