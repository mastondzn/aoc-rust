use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let rucksacks = input.lines().map(|line| {
        let length = line.len() / 2;
        let (left, right) = line.split_at(length);

        let left = left.chars().collect::<HashSet<_>>();
        let right = right.chars().collect::<HashSet<_>>();

        (left, right)
    });

    let total = rucksacks.fold(0, |acc, (left, right)| {
        let intersection = left.intersection(&right);

        acc + intersection.fold(0, |acc2, c| {
            acc2 as u32 + chars.iter().position(|&x| x == *c).unwrap() as u32
        })
    });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<char> = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let lines: Vec<_> = input.lines().collect::<Vec<_>>();
    let groups = lines.chunks(3).map(|chunk| {
        let left = chunk[0].chars().collect::<HashSet<_>>();
        let middle = chunk[1].chars().collect::<HashSet<_>>();
        let right = chunk[2].chars().collect::<HashSet<_>>();

        (left, middle, right)
    });

    let total = groups.fold(0, |acc, (left, middle, right)| {
        let intersection = left
            .intersection(&middle)
            .map(|&c| c)
            .collect::<HashSet<_>>();

        let intersection = intersection.intersection(&right);

        acc + intersection.fold(0, |acc2, c| {
            acc2 as u32 + chars.iter().position(|&x| x == *c).unwrap() as u32
        })
    });

    Some(total)
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
