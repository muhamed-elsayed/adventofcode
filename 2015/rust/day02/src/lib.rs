use std::{fs, time::Instant};

pub fn benhmark(name: &str, proc: &dyn Fn(&str) -> String) {
    let input = fs::read_to_string("./input.txt").unwrap();

    let time = Instant::now();
    let answer = proc(&input);
    let duration = time.elapsed().as_micros();

    println!("{name}: {answer}");
    println!("Duration: {duration} μs");
}

pub fn proc_part1(input: &str) -> String {
    let mut calc = 0;
    for line in input.lines() {
        let mut dimensions = [0; 3];
        for (i, v) in line.split('x').enumerate() {
            let value = v.parse::<u32>().unwrap();
            dimensions[i] = value;
        }

        dimensions.sort();
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2]; // largest

        let out = (2 * l * w + 2 * w * h + 2 * h * l) + l * w;

        calc += out;
    }

    calc.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let mut calc = 0;
    for line in input.lines() {
        let mut dimensions = [0; 3];
        for (i, v) in line.split('x').enumerate() {
            let value = v.parse::<u32>().unwrap();
            dimensions[i] = value;
        }

        dimensions.sort();
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        let out = (l + l + w + w) + (l * w * h);

        calc += out;
    }

    calc.to_string()
}

#[cfg(test)]
mod proc_part1 {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(proc_part1("2x3x4"), "58");
    }

    #[test]
    fn test02() {
        assert_eq!(proc_part1("1x1x10"), "43");
    }

    #[test]
    fn test_proc_part1() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part1(&input), "1606483");
    }
}

#[cfg(test)]
mod proc_part2 {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(proc_part2("2x3x4"), "34");
    }

    #[test]
    fn test02() {
        assert_eq!(proc_part2("1x1x10"), "14");
    }

    #[test]
    fn test_proc_part2() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(proc_part2(&input), "3842356");
    }
}
