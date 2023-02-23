use day01::proc_part1;

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", proc_part1(&input));
}
