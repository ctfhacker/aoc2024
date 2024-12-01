#![feature(thread_id_value)]
#![deny(clippy::pedantic)]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT: &str = include_str!("../input");

fn part1(input: &str) -> Result<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut items = line.split_whitespace();
        left.push(items.next().ok_or("No left")?.parse::<i32>()?);
        right.push(items.next().ok_or("No right")?.parse::<i32>()?);
    }

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    Ok(result)
}

fn part2(input: &str) -> Result<u32> {
    let mut sums = vec![0_u32; 128 * 1024];

    let mut left = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut items = line.split_whitespace();
        left.push(items.next().ok_or("No left")?.parse::<u32>()?);
        let val = items.next().ok_or("No right")?.parse::<usize>()?;
        sums[val] += 1;
    }

    Ok(left.iter().map(|index| index * sums[*index as usize]).sum())
}

fn main() {
    println!("Part 1: {:?}", part1(INPUT));
    println!("Part 2: {:?}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT: &str = include_str!("../test_input");

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 31);
    }
}
