use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut paper_positions = HashSet::new();
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().enumerate().for_each(|(y, line)| {
        let chars = line.chars();

        chars.enumerate().for_each(|(x, char)| {
            if char == '@' {
                paper_positions.insert((x as i32, y as i32));
            }
        });
    });

    paper_positions
}

const POSITION_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let paper_positions = parse_input(input);

    Some(
        paper_positions
            .iter()
            .filter(|(x, y)| {
                POSITION_OFFSETS
                    .iter()
                    .filter(|(x_offset, y_offset)| {
                        paper_positions.contains(&(x + x_offset, y + y_offset))
                    })
                    .count()
                    < 4
            })
            .count() as u64,
    )
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
