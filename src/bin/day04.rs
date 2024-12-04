
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(PartialEq)]
enum Letter {
    X,
    M,
    A,
    S
}
#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagonalLeftUp,
    DiagonalLeftDown,
    DiagonalRightUp,
    DiagonalRightDown
}

#[derive(EnumIter)]
enum MasDirection {
    Up,
    Down,
    Left,
    Right
}

// yes this is a dumb just way too verbose way to do this
// i just wanted to go fast
// also -- you must run this in release mode to get rid of underflow errors for usize
fn main() {
    let input: String = std::fs::read_to_string("day04.txt").unwrap();
    let lines: Vec<Vec<Letter>> = input.lines().map(|line| line.chars().map(|c| {
        match c {
            'X' => Letter::X,
            'M' => Letter::M,
            'A' => Letter::A,
            'S' => Letter::S,
            _ => unreachable!()
        }
    }).collect()).collect();

    let mut count = 0;
    let mut x_count = 0;
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        for j in 0..line.len() {
            let letter = line.get(j).unwrap();
            if *letter != Letter::X {
                if *letter == Letter::M {
                    for direction in MasDirection::iter() {
                        if search2(&lines, i, j, direction).unwrap_or(false) {
                            x_count += 1;
                        }
                    }
                }
                continue;
            }
            for direction in Direction::iter() {
                if search(&lines, i, j, direction) {
                    count += 1;
                }
            }
        }
    }
    dbg!(count);
    dbg!(x_count);
}

fn search2(lines: &Vec<Vec<Letter>>, i: usize, j: usize, direction: MasDirection) -> Option<bool> {
    let line = lines.get(i).unwrap();
    match direction {
        MasDirection::Up => {
            let l0 = lines.get(i - 1)?;
            let l1 = lines.get(i - 2)?;
            if *line.get(j + 2)? != Letter::M {
                return Some(false);
            }
            if *l1.get(j)? != Letter::S {
                return Some(false);
            }
            if *l1.get(j + 2)? != Letter::S {
                return Some(false);
            }
            if *l0.get(j + 1)? != Letter::A {
                return Some(false);
            }
        },
        MasDirection::Down => {
            let l0 = lines.get(i + 1)?;
            let l1 = lines.get(i + 2)?;
            if *line.get(j + 2)? != Letter::M {
                return Some(false);
            }
            if *l1.get(j)? != Letter::S {
                return Some(false);
            }
            if *l1.get(j + 2)? != Letter::S {
                return Some(false);
            }
            if *l0.get(j + 1)? != Letter::A {
                return Some(false);
            }
        },
        MasDirection::Left => {
            let l0 = lines.get(i + 1)?;
            let l1 = lines.get(i + 2)?;
            if *l1.get(j)? != Letter::M {
                return Some(false);
            }
            if *line.get(j - 2)? != Letter::S {
                return Some(false);
            }
            if *l1.get(j - 2)? != Letter::S {
                return Some(false);
            }
            if *l0.get(j - 1)? != Letter::A {
                return Some(false);
            }
        },
        MasDirection::Right => {
            let l0 = lines.get(i + 1)?;
            let l1 = lines.get(i + 2)?;
            if *l1.get(j)? != Letter::M {
                return Some(false);
            }
            if *line.get(j + 2)? != Letter::S {
                return Some(false);
            }
            if *l1.get(j + 2)? != Letter::S {
                return Some(false);
            }
            if *l0.get(j + 1)? != Letter::A {
                return Some(false);
            }
        },
    }
    Some(true)
}

fn search(lines: &Vec<Vec<Letter>>, i: usize, j: usize, direction: Direction) -> bool {
    match direction {
        Direction::Up => {
            if i < 3 {
                return false;
            }
            if *lines.get(i - 1).unwrap().get(j).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i - 2).unwrap().get(j).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i - 3).unwrap().get(j).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::Down => {
            if i > (lines.len() - 4) {
                return false;
            }
            if *lines.get(i + 1).unwrap().get(j).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i + 2).unwrap().get(j).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i + 3).unwrap().get(j).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::Left => {
            if j < 3 {
                return false;
            }
            if *lines.get(i).unwrap().get(j - 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i).unwrap().get(j - 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i).unwrap().get(j - 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::Right => {
            if j > (lines.get(i).unwrap().len() - 4) {
                return false;
            }
            if *lines.get(i).unwrap().get(j + 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i).unwrap().get(j + 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i).unwrap().get(j + 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::DiagonalRightDown => {
            if j > (lines.get(i).unwrap().len() - 4) {
                return false;
            }
            if i > (lines.len() - 4) {
                return false;
            }
            if *lines.get(i + 1).unwrap().get(j + 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i + 2).unwrap().get(j + 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i + 3).unwrap().get(j + 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::DiagonalRightUp => {
            if j > (lines.get(i).unwrap().len() - 4) {
                return false;
            }
            if i < 3 {
                return false;
            }
            if *lines.get(i - 1).unwrap().get(j + 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i - 2).unwrap().get(j + 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i - 3).unwrap().get(j + 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::DiagonalLeftDown => {
            if j < 3 {
                return false;
            }
            if i > (lines.len() - 4) {
                return false;
            }
            if *lines.get(i + 1).unwrap().get(j - 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i + 2).unwrap().get(j - 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i + 3).unwrap().get(j - 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        },
        Direction::DiagonalLeftUp => {
            if j < 3 {
                return false;
            }
            if i < 3 {
                return false;
            }
            if *lines.get(i - 1).unwrap().get(j - 1).unwrap() != Letter::M {
                return false;
            }
            if *lines.get(i - 2).unwrap().get(j - 2).unwrap() != Letter::A {
                return false;
            }
            if *lines.get(i - 3).unwrap().get(j - 3).unwrap() != Letter::S {
                return false;
            }
            return true;
        }
    }
}