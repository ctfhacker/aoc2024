#![feature(array_windows)]
#![warn(clippy::pedantic)]

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT: &str = include_str!("../input");

fn part1(input: &str) -> i32 {
    let mut safe = 0;

    'next_line: for line in input.split('\n').filter(|line| !line.is_empty()) {
        let items: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let mut ascending = true;
        let mut descending = true;

        for [x, y] in items.array_windows() {
            let diff = x - y;
            if ![1, 2, 3].contains(&diff.abs()) {
                continue 'next_line;
            }

            if diff < 0 {
                descending = false;
            }
            if diff > 0 {
                ascending = false;
            }
        }

        safe += i32::from(ascending || descending);
    }

    safe
}

fn part2(input: &str) -> i32 {
    let mut safe = 0;

    for line in input.split('\n').filter(|line| !line.is_empty()) {
        let items: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let mut is_safe = false;
        'next_index: for missing_index in 0..items.len() {
            let mut indexes: Vec<usize> = (0..items.len()).collect();
            indexes.remove(missing_index);

            let mut ascending = true;
            let mut descending = true;

            for [x, y] in indexes.array_windows() {
                let diff = items[*x] - items[*y];

                if ![1, 2, 3].contains(&diff.abs()) {
                    continue 'next_index;
                }

                if diff < 0 {
                    descending = false;
                }
                if diff > 0 {
                    ascending = false;
                }
            }

            is_safe = ascending || descending;
            if is_safe {
                break;
            }
        }

        safe += i32::from(is_safe);
    }

    safe
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
    fn test_day2_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
