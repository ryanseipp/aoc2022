pub fn part_one(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();
    return contents
        .split(|&i| i == "")
        .map(|elf| elf.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap();
}

pub fn part_one_imperative(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();

    let mut max_calories: i32 = 0;
    let mut elf_sum: i32 = 0;

    for line in contents {
        if line == "" {
            if elf_sum > max_calories {
                max_calories = elf_sum;
            }
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    return max_calories;
}

pub fn part_one_bitwise(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut max: i32 = 0;
    let mut sum: i32 = 0;
    let mut current: i32 = 0;

    for byte in bytes {
        if *byte != b'\n' {
            current *= 10;
            current += (byte - b'0') as i32;
        } else if current == 0 && sum > max {
            max = sum;
            sum = 0;
        } else if current == 0 {
            sum = 0;
        } else {
            sum += current;
            current = 0;
        }
    }

    return max;
}

pub fn part_two(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();
    let mut sums: Vec<i32> = contents
        .split(|&i| i == "")
        .map(|elf| elf.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .collect();

    sums.sort();
    return sums.iter().rev().take(3).sum();
}

pub fn part_two_imperative(input: &str) -> i32 {
    let contents: Vec<&str> = input.split('\n').collect();

    let mut top_three_calories: [i32; 3] = [0; 3];
    let mut elf_sum: i32 = 0;

    for line in contents {
        if line == "" {
            let mut shift_right = false;
            let mut prev_value: i32 = 0;

            for index in 0..top_three_calories.len() {
                if shift_right {
                    let temp = top_three_calories[index];
                    top_three_calories[index] = prev_value;
                    prev_value = temp;
                } else if elf_sum > top_three_calories[index] {
                    shift_right = true;
                    prev_value = top_three_calories[index];
                    top_three_calories[index] = elf_sum;
                }
            }

            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    return top_three_calories.iter().sum();
}

pub fn part_two_bitwise(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut top_three_calories: [i32; 3] = [0; 3];
    let mut sum: i32 = 0;
    let mut current: i32 = 0;

    for byte in bytes {
        if *byte != b'\n' {
            current *= 10;
            current += (byte - b'0') as i32;
        } else if current == 0 {
            let mut shift_right = false;
            let mut prev_value: i32 = 0;

            for index in 0..top_three_calories.len() {
                if shift_right {
                    let temp = top_three_calories[index];
                    top_three_calories[index] = prev_value;
                    prev_value = temp;
                } else if sum > top_three_calories[index] {
                    shift_right = true;
                    prev_value = top_three_calories[index];
                    top_three_calories[index] = sum;
                }
            }

            sum = 0;
        } else {
            sum += current;
            current = 0;
        }
    }

    return top_three_calories[0] + top_three_calories[1] + top_three_calories[2];
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
        assert_eq!(71924, part_one(&get_input()));
    }

    #[test]
    fn part_one_imperative_returns_correct_result() {
        assert_eq!(71924, part_one_imperative(&get_input()));
    }

    #[test]
    fn part_one_bitwise_returns_correct_result() {
        assert_eq!(71924, part_one_bitwise(&get_input()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        assert_eq!(210406, part_two(&get_input()));
    }

    #[test]
    fn part_two_imperative_returns_correct_result() {
        assert_eq!(210406, part_two_imperative(&get_input()));
    }

    #[test]
    fn part_two_bitwise_returns_correct_result() {
        assert_eq!(210406, part_two_bitwise(&get_input()));
    }
}
