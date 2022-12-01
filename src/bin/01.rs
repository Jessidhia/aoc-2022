use itertools::Itertools;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list0, separated_list1},
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list0(
        many1(line_ending),
        separated_list1(
            line_ending,
            map_res(digit1, |input| u32::from_str_radix(input, 10)),
        ),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let list = parse(input).unwrap().1;
    let totals = list.into_iter().map(|item| item.into_iter().sum::<u32>());
    Some(totals.max().unwrap_or(0))
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = parse(input).unwrap().1;
    let totals = list.into_iter().map(|item| item.into_iter().sum::<u32>());
    let top3 = totals.sorted_unstable().rev().take(3);
    Some(top3.sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
