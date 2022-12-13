use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

enum Instruction {
    NoOp,
    AddX(i32),
}

#[derive(Debug)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing instruction")
    }
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(_: ParseIntError) -> Self {
        ParseInstructionError
    }
}

impl Error for ParseInstructionError {}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        return match parts[0] {
            "noop" => Ok(Instruction::NoOp),
            "addx" => Ok(Instruction::AddX(parts[1].parse()?)),
            _ => Err(ParseInstructionError),
        };
    }
}

enum CycleOp {
    Normal,
    Important,
    ImportantTakeOld(i32),
}

fn get_cycle_op(old_cycle: i32, new_cycle: i32) -> CycleOp {
    let important_cycle: [i32; 6] = [20, 60, 100, 140, 180, 220];

    for cycle in important_cycle {
        if old_cycle < cycle && new_cycle == cycle {
            return CycleOp::Important;
        }
        if old_cycle < cycle && new_cycle > cycle {
            return CycleOp::ImportantTakeOld(cycle);
        }
    }

    return CycleOp::Normal;
}

pub fn part_one(input: &str) -> i32 {
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut sum: i32 = 0;

    input
        .lines()
        .filter_map(|l| Instruction::from_str(l).ok())
        .for_each(|instruction| match instruction {
            Instruction::NoOp => {
                let new_cycle = cycle + 1;
                match get_cycle_op(cycle, new_cycle) {
                    CycleOp::Normal => cycle = new_cycle,
                    CycleOp::Important => {
                        cycle = new_cycle;
                        sum += register_x * cycle;
                    }
                    CycleOp::ImportantTakeOld(c) => {
                        cycle = new_cycle;
                        sum += register_x * c;
                    }
                }
            }
            Instruction::AddX(x) => {
                let new_cycle = cycle + 2;
                match get_cycle_op(cycle, new_cycle) {
                    CycleOp::Normal => {
                        cycle = new_cycle;
                        register_x += x;
                    }
                    CycleOp::Important => {
                        cycle = new_cycle;
                        sum += register_x * cycle;
                        register_x += x;
                    }
                    CycleOp::ImportantTakeOld(c) => {
                        cycle = new_cycle;
                        sum += register_x * c;
                        register_x += x;
                    }
                }
            }
        });

    return sum;
}

fn draw_crt(cycle: i32, register_x: i32, crt: &mut String) {
    let cycle_mod = cycle % 40;
    if ((register_x - 1)..=(register_x + 1)).contains(&cycle_mod) {
        crt.push('#');
    } else {
        crt.push('.');
    }
    if cycle_mod + 1 == 40 {
        crt.push('\n');
    }
}

pub fn part_two(input: &str) -> String {
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut sum: i32 = 0;
    let mut crt: String = "".to_string();

    input
        .lines()
        .filter_map(|l| Instruction::from_str(l).ok())
        .for_each(|instruction| match instruction {
            Instruction::NoOp => {
                draw_crt(cycle, register_x, &mut crt);
                let new_cycle = cycle + 1;
                match get_cycle_op(cycle, new_cycle) {
                    CycleOp::Normal => cycle = new_cycle,
                    CycleOp::Important => {
                        cycle = new_cycle;
                        sum += register_x * cycle;
                    }
                    CycleOp::ImportantTakeOld(c) => {
                        cycle = new_cycle;
                        sum += register_x * c;
                    }
                }
            }
            Instruction::AddX(x) => {
                draw_crt(cycle, register_x, &mut crt);
                draw_crt(cycle + 1, register_x, &mut crt);

                let new_cycle = cycle + 2;
                match get_cycle_op(cycle, new_cycle) {
                    CycleOp::Normal => {
                        cycle = new_cycle;
                        register_x += x;
                    }
                    CycleOp::Important => {
                        cycle = new_cycle;
                        sum += register_x * cycle;
                        register_x += x;
                    }
                    CycleOp::ImportantTakeOld(c) => {
                        cycle = new_cycle;
                        sum += register_x * c;
                        register_x += x;
                    }
                }
            }
        });

    return crt;
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_example() -> &'static str {
        return "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
    }

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn part_one_computes_example() {
        assert_eq!(13140, part_one(get_example()));
    }

    #[test]
    fn part_two_computes_example() {
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n",
            part_two(get_example())
        );
    }

    #[test]
    fn part_one_computes_correct_result() {
        assert_eq!(15020, part_one(&get_input()));
    }

    #[test]
    fn part_two_computes_correct_result() {
        assert_eq!(
            "####.####.#..#..##..#....###...##..###..\n#....#....#..#.#..#.#....#..#.#..#.#..#.\n###..###..#..#.#....#....#..#.#..#.#..#.\n#....#....#..#.#.##.#....###..####.###..\n#....#....#..#.#..#.#....#....#..#.#....\n####.#.....##...###.####.#....#..#.#....\n",
            part_two(&get_input())
        );
    }
}
