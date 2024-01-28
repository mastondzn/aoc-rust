advent_of_code::solution!(4);

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn from_str(input: &str) -> Self {
        let (start, end) = input.split_once('-').unwrap();

        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Self::new(start, end)
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();

            (Range::from_str(left), Range::from_str(right))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);

    let amount_one_contains_other = pairs.iter().fold(0, |acc, (left, right)| {
        let one_contains = left.contains(right) || right.contains(left);

        if one_contains {
            acc + 1
        } else {
            acc
        }
    });

    Some(amount_one_contains_other)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse_input(input);

    let amount_overlapping =
        pairs
            .iter()
            .fold(0, |acc, (left, right)| match left.overlaps(right) {
                true => acc + 1,
                false => acc,
            });

    Some(amount_overlapping)
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
