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

    return " ".to_string();
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
}

#[cfg(test)]
mod proc_part2 {
    use super::proc_part2;

    #[test]
    fn test01() {
        assert_eq!(proc_part2(")"), "1");
        assert_eq!(proc_part2("()())"), "5");
    }
}
