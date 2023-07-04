use std::{fs, rc::Rc, str::FromStr};

pub fn process_part1(lines: &[String]) {
    // Transform each line to a round moves
    let rounds = lines
        .iter()
        .map(|l| RoundMoves::try_from(l.as_str()).ok())
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    let score = rounds.iter().map(|r| r.calc_result()).sum::<u32>();
    print!("{score}");
}

pub fn process_part2(lines: &[String]) {
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

#[derive(Clone, Copy)]
#[repr(u32)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Move {
    fn wins(&self, other: &Self) -> u32 {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => 6,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => 3,
            (_, _) => 0,
        }
    }
}

struct RoundMoves(Move, Move);
impl RoundMoves {
    fn calc_result(&self) -> u32 {
        let RoundMoves(other, you) = self;
        (*you as u32) + (you.wins(other))
    }
}

impl TryFrom<char> for Move {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            c => Err(format!("Couldn't extract Move: {c} is not a valid value").to_string()),
        }
    }
}

impl TryFrom<&str> for RoundMoves {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let moves = s
            .trim()
            .chars()
            .map(|c| Move::try_from(c))
            .filter(|m| m.is_ok())
            .map(|m| m.unwrap())
            .collect::<Vec<_>>();

        if moves.len() != 2 {
            Err("Couldn't extract RoundMoves")
        } else {
            Ok(Self(*moves.first().unwrap(), *moves.last().unwrap()))
        }
    }
}
