use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Clone, Debug, Copy, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn versus(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,

            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,

            _ => Outcome::Draw,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    const VALUES: [Self; 3] = [Move::Rock, Move::Paper, Move::Scissors];
}

fn calculate_score<'a, 'b>(moves: impl Iterator<Item = (&'a Move, &'b Move)>) -> u32 {
    moves.fold(0, |score, (left, right)| {
        score + right.versus(&left).score() + right.score()
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let left_decode_map: HashMap<_, _> =
        [("A", Move::Rock), ("B", Move::Paper), ("C", Move::Scissors)]
            .iter()
            .cloned()
            .collect();

    let right_decode_map: HashMap<_, _> =
        [("X", Move::Rock), ("Y", Move::Paper), ("Z", Move::Scissors)]
            .iter()
            .cloned()
            .collect();

    let moves = input.lines().map(|line| {
        let (left, right) = line.split_once(" ").unwrap();

        (
            left_decode_map.get(left).unwrap(),
            right_decode_map.get(right).unwrap(),
        )
    });

    Some(calculate_score(moves))
}

pub fn part_two(input: &str) -> Option<u32> {
    let left_decode_map: HashMap<_, _> =
        [("A", Move::Rock), ("B", Move::Paper), ("C", Move::Scissors)]
            .iter()
            .cloned()
            .collect();

    let right_decode_map: HashMap<_, _> = [
        ("X", Outcome::Lose),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Win),
    ]
    .iter()
    .cloned()
    .collect();

    let moves = input.lines().map(|line| {
        let (left, right) = line.split_once(" ").unwrap();
        let left_move = left_decode_map.get(left).unwrap();
        let desired_outcome = right_decode_map.get(right).unwrap();

        let right_move = Move::VALUES
            .iter()
            .find(|move_| move_.versus(left_move) == *desired_outcome)
            .unwrap();

        (left_move, right_move)
    });

    Some(calculate_score(moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
