advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|range_string| {
            let (start, end) = range_string.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_total: u64 = 0;

    parse_input(input).iter().for_each(|&(start, end)| {
        for id in start..(end + 1) {
            let string_id = id.to_string();
            if string_id.len() % 2 == 1 {
                continue;
            }

            let (left, right) = string_id.split_at(string_id.len() / 2);
            if left == right {
                invalid_total += id
            };
        }
    });

    Some(invalid_total)
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
        assert_eq!(result.unwrap(), 1227775554);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
