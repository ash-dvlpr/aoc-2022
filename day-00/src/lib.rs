use std::{fs, rc::Rc};

pub fn process_part1(lines: Rc<[String]>) {
    todo!("Process Input of Part 1");
    
    lines.iter().for_each(|l| {
        println!("{l}")
    });
}

pub fn process_part2(lines: Rc<[String]>) {
    todo!("Process Input of Part 2");
    
    lines.iter().for_each(|l| {
        println!("{l}")
    });
}

// =============== Common Code ===============
pub fn extract_lines(file_path: &str) -> Rc<[String]> {
    let lines = fs::read_to_string(file_path)
        .unwrap()
        .lines().map(|l| {l.to_owned()})
        .collect::<Vec<_>>();
    
    Rc::from(lines)
}
