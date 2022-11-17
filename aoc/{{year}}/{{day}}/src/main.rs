use {{project-name}}::{part1, part2};
use utils::output::Output;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("part1: {}", part1(input).output());
    println!("part2: {}", part2(input).output());
}
