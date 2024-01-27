advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = parse_input(input);

    let mut most = 0;
    for elf in elves {
        let total = elf.iter().fold(0, |acc, x| acc + x);
        if total > most {
            most = total;
        }
    }

    Some(most)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = parse_input(input);

    let mut top_three = [0, 0, 0];
    for elf in elves {
        let total = elf.iter().fold(0, |acc, x| acc + x);

        if total > top_three[0] {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = total;
        } else if total > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = total;
        } else if total > top_three[2] {
            top_three[2] = total;
        }
    }

    println!("{:?}", top_three);

    let top_three_total = top_three.iter().fold(0, |acc, x| acc + x);

    Some(top_three_total)
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
