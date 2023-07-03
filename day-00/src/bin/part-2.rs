use std::fs;
use day_00::*;

fn main() {
    let file = fs::read_to_string("./input2.txt").unwrap();
    process_part2(&file);
}