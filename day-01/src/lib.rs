use std::{fs, rc::Rc};

pub fn process_part1(lines: &[String]) {
    // Separate the lines into groups by elf
    let elves = separate_elves_lines(&lines);

    // Sum all the calories of each elf
    let calories = calculate_elves_calories(&elves);

    // Find the biggest calorie count (Result)
    println!("{}", calories.iter().max().unwrap());
}

pub fn process_part2(lines: Rc<[String]>) {
    // Separate the lines into groups by elf
    let elves = separate_elves_lines(&lines);

    // Sum all the calories of each elf
    let calories = calculate_elves_calories(&elves);

    todo!("Process Input of Part 2");
}

// =============== Common Code ===============
pub fn extract_lines(file_path: &str) -> Rc<[String]> {
    let lines = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<_>>();

    Rc::from(lines)
}

fn separate_elves_lines<'a>(lines: &'a[String]) -> Vec<&'a [String]> {
    // Every group of succesive lines are all lines from a single elf.
    // If there's an empty line, there was a double line break.
    // Find those breaks, and use them to create slices.

    let line_breaks = lines
        .clone()
        .iter()
        .enumerate()
        .filter(|(i, l)| l.is_empty() || *i == lines.len() - 1)
        .map(|(i, _l)| i)
        .collect::<Vec<_>>();

    // Separate into slices, one per elf
    let elves = &line_breaks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let i = if i == 0 {
                0 as usize
            } else {
                *line_breaks.get(i - 1).unwrap() + (1 as usize)
            };

            let b = if b == line_breaks.last().unwrap() {
                lines.len()
            } else {
                *b
            };

            &lines[i..b]
        })
        .collect::<Vec<&[String]>>();

    elves.to_owned()
}

fn calculate_elves_calories(elves: &[&[String]]) -> Vec<i64> {
    elves
        .iter()
        .map(|elf| {
            elf.iter()
                // Convert to integers
                .map(|l| l.parse::<i64>().unwrap())
                // Reduce by adition
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .collect::<Vec<_>>()
}
