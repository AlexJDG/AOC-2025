advent_of_code::solution!(3);

pub fn parse_input(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u16)
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input).iter().fold(0u64, |total, bank| {
        let biggest = bank[0..bank.len() - 1].iter().max().unwrap();
        let biggestIndex = bank.iter().position(|battery| battery == biggest).unwrap();

        total + ((biggest * 10) + bank[biggestIndex + 1..bank.len()].iter().max().unwrap()) as u64
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
