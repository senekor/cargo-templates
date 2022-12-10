use {{project-name}}::{part1, part2};
use utils::output::Output;

static INPUT: &str = include_str!("../input/input.txt");

fn main() {
    println!("part1: {}", part1(INPUT).output());
    println!("part2: {}", part2(INPUT).output());
}
