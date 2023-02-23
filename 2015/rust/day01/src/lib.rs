pub fn proc_part1(input: &str) -> String {
    let mut floor = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
    }
    floor.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let mut floor = 0;
    for (index, ch) in input.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
        if floor == -1 {
            let pos = index + 1;
            return pos.to_string();
        }
    }
    unreachable!("");
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
