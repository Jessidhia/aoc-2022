use std::{iter::Sum, str::FromStr};

use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum MoveComparison {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Move {
    /// Returns the Move that would win over this move
    fn winning_move(&self) -> Move {
        use Move::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    /// Returns the Move that would lose over this move
    fn losing_move(&self) -> Move {
        use Move::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn compare(&self, other: &Self) -> MoveComparison {
        use MoveComparison::*;

        if self == other {
            Draw
        } else if self == &other.winning_move() {
            Win
        } else {
            Loss
        }

        // use Move::*;
        // match self {
        //     Rock => match other {
        //         Rock => Draw,
        //         Paper => Loss,
        //         Scissors => Win,
        //     },
        //     Paper => match other {
        //         Rock => Win,
        //         Paper => Draw,
        //         Scissors => Loss,
        //     },
        //     Scissors => match other {
        //         Rock => Loss,
        //         Paper => Win,
        //         Scissors => Draw,
        //     },
        // }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Play(Move, Move);

impl From<Play> for u32 {
    fn from(Play(opponent, player): Play) -> Self {
        let cmp = player.compare(&opponent);
        // println!(
        //     "{:?} vs {:?} ({:?}), score {} for {:?} + {} for {:?}",
        //     player, opponent, cmp, player as Self, player, cmp as Self, cmp
        // );
        player as Self + cmp as Self
    }
}

impl Sum<Play> for u32 {
    fn sum<I: Iterator<Item = Play>>(iter: I) -> Self {
        iter.map_into::<u32>().sum()
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Move {} not recognized", s)),
        }
    }
}

impl FromStr for MoveComparison {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MoveComparison::Loss),
            "Y" => Ok(MoveComparison::Draw),
            "Z" => Ok(MoveComparison::Win),
            _ => Err(format!("MoveComparison {} not recognized", s)),
        }
    }
}

fn parse_part_one(input: &str) -> IResult<&str, Vec<Play>> {
    let make_rps_move = || map_res(alpha1, |m: &str| m.parse::<Move>());
    let rps_play = map(
        separated_pair(make_rps_move(), space1, make_rps_move()),
        |(opponent, player)| Play(opponent, player),
    );

    separated_list0(line_ending, rps_play)(input)
}

fn parse_part_two(input: &str) -> IResult<&str, Vec<(Move, MoveComparison)>> {
    let rps_play = separated_pair(
        map_res(alpha1, |m: &str| m.parse::<Move>()),
        space1,
        map_res(alpha1, |m: &str| m.parse::<MoveComparison>()),
    );

    separated_list0(line_ending, rps_play)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let list = parse_part_one(input).unwrap().1;
    Some(list.into_iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = parse_part_two(input).unwrap().1;
    Some(
        list.into_iter()
            .map(|(opponent, state)| {
                Play(
                    opponent,
                    match state {
                        MoveComparison::Win => opponent.winning_move(),
                        MoveComparison::Draw => opponent,
                        MoveComparison::Loss => opponent.losing_move(),
                    },
                )
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
