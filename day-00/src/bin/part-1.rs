use std::fs;
use day_00::*;

fn main() {
    let file = fs::read_to_string("./input1.txt").unwrap();
    process_part1(&file);
}