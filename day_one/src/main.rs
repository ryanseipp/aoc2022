use day_one::{part_one, part_two};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
    println!("Part 1: {}", part_one(input.as_str()));
    println!("Part 2: {}", part_two(input.as_str()));
}
