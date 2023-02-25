use std::collections::HashSet;
use std::{fs, time::Instant};

pub fn benhmark(name: &str, proc: &dyn Fn(&str) -> String) {
    let input = fs::read_to_string("./input.txt").unwrap();

    let time = Instant::now();
    let answer = proc(&input);
    let duration = time.elapsed().as_micros();

    println!("{name}: {answer}");
    println!("Duration: {duration} μs");
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .into_iter()
        .map(|c| match c {
            '^' => Direction::N,
            '>' => Direction::E,
            'v' => Direction::S,
            '<' => Direction::W,
            _ => panic!("Invalid direction: {}", c),
        })
        .collect()
}

pub fn proc_part1(input: &str) -> String {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();
    let path = parse_input(input);

    let (mut sx, mut sy) = (0, 0);
    visited_houses.insert((0, 0));
    for step in path {
        match step {
            Direction::N => sy += 1,
            Direction::E => sx += 1,
            Direction::S => sy -= 1,
            Direction::W => sx -= 1,
        }
        visited_houses.insert((sx, sy));
    }

    visited_houses.len().to_string()
}

pub fn proc_part2(input: &str) -> String {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();
    let path = parse_input(input);

    let (mut sx, mut sy) = (0, 0);
    let (mut rx, mut ry) = (0, 0);
    visited_houses.insert((0, 0));
    for step in (0..path.len()).step_by(2) {
        match path[step] {
            Direction::N => sy += 1,
            Direction::E => sx += 1,
            Direction::S => sy -= 1,
            Direction::W => sx -= 1,
        }

        match path[step + 1] {
            Direction::N => ry += 1,
            Direction::E => rx += 1,
            Direction::S => ry -= 1,
            Direction::W => rx -= 1,
        }

        visited_houses.insert((sx, sy));
        visited_houses.insert((rx, ry));
    }

    visited_houses.len().to_string()
}

#[cfg(test)]
mod proc_part1 {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![Direction::N], parse_input("^"));
        assert_eq!(vec![Direction::E], parse_input(">"));
        assert_eq!(vec![Direction::S], parse_input("v"));
        assert_eq!(vec![Direction::W], parse_input("<"));
        assert_eq!(
            vec![
                Direction::N,
                Direction::E,
                Direction::S,
                Direction::W,
                Direction::S,
            ],
            parse_input("^>v<v")
        );
    }
    #[test]
    fn test01() {
        assert_eq!(proc_part1(">"), "2");
    }

    #[test]
    fn test02() {
        assert_eq!(proc_part1("^>v<"), "4");
    }

    #[test]
    fn test03() {
        assert_eq!(proc_part1("^v^v^v^v^v"), "2");
    }

    #[test]
    fn test_proc_part1() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part1(&input), "2081");
    }
}

#[cfg(test)]
mod proc_part2 {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(proc_part2("^v"), "3");
    }

    #[test]
    fn test02() {
        assert_eq!(proc_part2("^>v<"), "3");
    }

    #[test]
    fn test03() {
        assert_eq!(proc_part2("^v^v^v^v^v"), "11");
    }

    #[test]
    fn test_proc_part2() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part2(&input), "2341");
    }
}
