use utils::{
    nom::{character::complete::newline, multi::separated_list0, IResult},
    parse::integer::i32,
};

pub(crate) fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(newline, i32)(input)
}

#[test]
fn test_numbers() {
    let input = "\
123
-456
";
    let (_, nums) = numbers(input).unwrap();
    assert_eq!(nums, vec![123, -456])
}
