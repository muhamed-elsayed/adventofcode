use std::{fs, time::Instant};

pub fn benhmark(name: String, proc: &dyn Fn(&str) -> String) {
pub fn benhmark(name: String, proc: &dyn Fn(&str) -> Option<i32>) {
    let input = fs::read_to_string("./input.txt").unwrap();

    let time = Instant::now();
    let answer = proc(&input);
    let duration = time.elapsed().as_micros();

    println!("{name}: {answer}");
    println!("Duration: {duration} μs");
}

pub fn proc_part1(input: &str) -> String {
    let floor = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invaild character: `{c}`"),
        })
        .sum::<i32>();

    floor.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let mut floor = 0;
    for (pos, ch) in input.chars().enumerate() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invaild character: `{ch}`"),
        };
        if floor == -1 {
            let pos = pos + 1;
            return pos.to_string();
        }
    }

    " ".to_string()
}

#[cfg(test)]
mod proc_part1 {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(proc_part1("(())"), "0");
        assert_eq!(proc_part1("()()"), "0");
    }

    #[test]
    fn test02() {
        assert_eq!(proc_part1("((("), "3");
        assert_eq!(proc_part1("(()(()("), "3");
    }

    #[test]
    fn test03() {
        assert_eq!(proc_part1("))((((("), "3");
    }

    #[test]
    fn test04() {
        assert_eq!(proc_part1("())"), "-1");
        assert_eq!(proc_part1("))("), "-1");
    }

    #[test]
    fn test05() {
        assert_eq!(proc_part1(")))"), "-3");
        assert_eq!(proc_part1(")())())"), "-3");
    }

    #[test]
    fn test_proc_part1() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part1(&input), "138");
    }
}

#[cfg(test)]
mod proc_part2 {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(proc_part2(")"), "1");
        assert_eq!(proc_part2("()())"), "5");
    }

    #[test]
    fn test_proc_part2() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part2(&input), "1771");
    }
}
