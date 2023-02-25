use day03::proc_part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", proc_part2(input));
}
