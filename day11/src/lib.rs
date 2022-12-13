use std::{collections::VecDeque, error::Error, fmt, num::ParseIntError, str::FromStr};

use evalexpr::{
    build_operator_tree, Context, ContextWithMutableVariables, HashMapContext, Node, Value,
};

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: Node,
    pub divisible_by: i64,
    pub throw_to: usize,
    pub throw_to_false: usize,
    pub inspection_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMonkeyError;

impl fmt::Display for ParseMonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing instruction")
    }
}

impl From<ParseIntError> for ParseMonkeyError {
    fn from(_: ParseIntError) -> Self {
        ParseMonkeyError
    }
}

impl Error for ParseMonkeyError {}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(6, lines.len());

        return Ok(Monkey {
            items: lines[1]
                .split(':')
                .nth(1)
                .ok_or(ParseMonkeyError)?
                .trim()
                .split(", ")
                .filter_map(|x| x.parse().ok())
                .collect(),
            operation: build_operator_tree(
                lines[2].split(':').nth(1).ok_or(ParseMonkeyError)?.trim(),
            )
            .unwrap(),
            divisible_by: lines[3]
                .split(' ')
                .last()
                .ok_or(ParseMonkeyError)?
                .parse()?,
            throw_to: lines[4]
                .split(' ')
                .last()
                .ok_or(ParseMonkeyError)?
                .parse()?,
            throw_to_false: lines[5]
                .split(' ')
                .last()
                .ok_or(ParseMonkeyError)?
                .parse()?,
            inspection_count: 0,
        });
    }
}

pub fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = Vec::default();
    let mut unparsed_monkey: String = String::default();

    for l in input.lines() {
        if l.is_empty() && unparsed_monkey.len() != 0 {
            monkeys.push(Monkey::from_str(&unparsed_monkey)?);
            unparsed_monkey.clear();
        } else {
            unparsed_monkey.push('\n');
            unparsed_monkey.push_str(l);
        }
    }

    if unparsed_monkey.len() > 0 {
        monkeys.push(Monkey::from_str(&unparsed_monkey)?);
    }

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_idx].items.pop_front() {
                let mut context = HashMapContext::new();
                context.set_value("old".to_string(), Value::from(item))?;
                let _ = &monkeys[monkey_idx]
                    .operation
                    .eval_empty_with_context_mut(&mut context)?;
                let new_worry_level =
                    (context.get_value("new").unwrap().as_int()? as f64 / 3.0).floor() as i64;

                if new_worry_level % monkeys[monkey_idx].divisible_by == 0 {
                    let throw_to = monkeys[monkey_idx].throw_to;
                    monkeys[throw_to].items.push_back(new_worry_level);
                } else {
                    let throw_to = monkeys[monkey_idx].throw_to_false;
                    monkeys[throw_to].items.push_back(new_worry_level);
                }

                monkeys[monkey_idx].inspection_count += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    return Ok(monkeys
        .iter()
        .take(2)
        .fold(1, |res, m| res * m.inspection_count));
}

pub fn part_two(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = Vec::default();
    let mut unparsed_monkey: String = String::default();

    for l in input.lines() {
        if l.is_empty() && unparsed_monkey.len() != 0 {
            monkeys.push(Monkey::from_str(&unparsed_monkey)?);
            unparsed_monkey.clear();
        } else {
            unparsed_monkey.push('\n');
            unparsed_monkey.push_str(l);
        }
    }

    if unparsed_monkey.len() > 0 {
        monkeys.push(Monkey::from_str(&unparsed_monkey)?);
    }

    let supermod = monkeys.iter().fold(1, |res, m| res * m.divisible_by);

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_idx].items.pop_front() {
                let mut context = HashMapContext::new();
                context.set_value("old".to_string(), Value::from(item))?;
                let _ = &monkeys[monkey_idx]
                    .operation
                    .eval_empty_with_context_mut(&mut context)?;
                let mut new_worry_level = context.get_value("new").unwrap().as_int()?;
                new_worry_level %= supermod;

                if new_worry_level % monkeys[monkey_idx].divisible_by == 0 {
                    let throw_to = monkeys[monkey_idx].throw_to;
                    monkeys[throw_to].items.push_back(new_worry_level);
                } else {
                    let throw_to = monkeys[monkey_idx].throw_to_false;
                    monkeys[throw_to].items.push_back(new_worry_level);
                }

                monkeys[monkey_idx].inspection_count += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    println!("\n{:?}", monkeys);

    return Ok(monkeys
        .iter()
        .take(2)
        .fold(1, |res, m| res * m.inspection_count));
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn part_one_computes_correct_result() {
        assert_eq!(0, part_one(&get_input()).unwrap());
    }

    #[test]
    fn part_two_computes_correct_result() {
        assert_eq!(0, part_two(&get_input()).unwrap());
    }
}
