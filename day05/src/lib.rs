#[derive(Debug)]
struct Instruction {
    move_count: usize,
    move_from: usize,
    move_to: usize,
}

fn parse_instruction(input: &str) -> Instruction {
    let numbers: Vec<usize> = input
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect();
    assert_eq!(3, numbers.len());

    return Instruction {
        move_count: numbers[0],
        move_from: numbers[1],
        move_to: numbers[2],
    };
}

pub fn part_one(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut first_blank_index: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            first_blank_index = i;
            break;
        }
    }

    let num_stacks = lines[first_blank_index - 1]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .last()
        .unwrap() as usize;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(num_stacks, Vec::new());

    for line in lines.iter().take(num_stacks - 1).rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    for line in lines.iter().skip(first_blank_index + 1) {
        let instruction = parse_instruction(line);
        for _ in 1..=instruction.move_count {
            let moved = stacks[instruction.move_from - 1].pop().unwrap();
            stacks[instruction.move_to - 1].push(moved);
        }
    }

    let mut top_of_stack: String = String::new();

    for mut stack in stacks {
        top_of_stack.push(stack.pop().unwrap());
    }

    return top_of_stack;
}

pub fn part_two(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut first_blank_index: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            first_blank_index = i;
            break;
        }
    }

    let num_stacks = lines[first_blank_index - 1]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .last()
        .unwrap() as usize;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(num_stacks, Vec::new());

    for line in lines.iter().take(num_stacks - 1).rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    for line in lines.iter().skip(first_blank_index + 1) {
        let instruction = parse_instruction(line);

        let mut moved: Vec<char> = Vec::new();
        for _ in 1..=instruction.move_count {
            moved.push(stacks[instruction.move_from - 1].pop().unwrap());
        }

        for &item in moved.iter().rev() {
            stacks[instruction.move_to - 1].push(item);
        }
    }

    let mut top_of_stack: String = String::new();

    for mut stack in stacks {
        top_of_stack.push(stack.pop().unwrap());
    }

    return top_of_stack;
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!("CWMTGHBDW", part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!("SSCGWJCRB", part_two(input.as_str()));
    }
}
