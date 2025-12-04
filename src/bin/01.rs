advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> Vec<(char, i32)> {
    input.lines().map(|line| {
        let (dir, count) = line.split_at(1);
        (dir.parse::<char>().unwrap(), count.parse::<i32>().unwrap())
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial:i32 = 50;
    let mut zero_count: u64 = 0;
    parse_input(input).iter().for_each(|&(dir, count)| {
        let mul = if dir == 'L' { -1 } else { 1 };

        dial = (dial + (count * mul)) % 100;

        dbg!(dial);

        if dial < 0 {
            dial += 100
        }

        if dial == 0 {
            zero_count += 1;
        }
    });

    Some(zero_count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 3u64);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
