use {{project-name}}::{part1, part2};
use utils::output::Output;

static SAMPLE: &str = include_str!("../input/sample.txt");

fn main() {
    println!("(sample) part1: {}", part1(SAMPLE).output());
    println!("(sample) part2: {}", part2(SAMPLE).output());
}
