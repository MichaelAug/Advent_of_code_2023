use crate::day::{Day, DayError};
use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: AhoCorasick = AhoCorasick::new([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ])
    .expect("Failed to inilialise AhoCorasick");
}

pub struct Day1;

impl Day for Day1 {
    fn part_1(&self, input: &str) -> Result<String, DayError> {
        let sum: u32 = input
            // Iterate over all lines
            .lines()
            // Ignore lines with errors, find first and last digits and concatenate them
            // this turns the list &str into list of u32
            .filter_map(|l| {
                format!(
                    "{}{}",
                    l.chars().find(|c| c.is_digit(10))?,
                    l.chars().rev().find(|c| c.is_digit(10))?
                )
                .parse::<u32>()
                .ok()
            })
            // Sum all values in the u32 list
            .sum();

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String, DayError> {
        let sum = input
            .lines()
            .filter_map(|l| {
                let (first, last) = find_digits(l).ok()?;

                // If needed convert word to a digit
                let first = word_to_digit(first).unwrap_or_else(|| first);
                let last = word_to_digit(last).unwrap_or_else(|| last);

                format!("{first}{last}").parse::<u32>().ok()
            })
            .sum::<u32>();

        Ok(sum.to_string())
    }
}

fn find_digits(input: &str) -> Result<(&str, &str), DayError> {
    let matches: Vec<&str> = AC
        .find_overlapping_iter(input)
        .map(|m| &input[m.start()..m.end()])
        .collect();

    matches
        .first()
        .and_then(|&first| matches.last().map(|&last| (first, last)))
        .ok_or_else(|| DayError::Iterator("find_digits: No digits matched".to_string()))
}

fn word_to_digit(input: &str) -> Option<&str> {
    match input {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        "ten" => Some("10"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part_1() {
        let input = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet";
        assert_eq!(Day1.part_1(input).unwrap(), "142");
    }
    #[test]
    fn part_2() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";

        assert_eq!(Day1.part_2(input).unwrap(), "281");
    }

    #[rstest]
    #[case("fivetwone", Ok(("five", "one")))]
    #[case("eighthree", Ok(("eight", "three")))]
    #[case("1twothree5", Ok(("1", "5")))]
    #[case("twototheoneandonetothe3", Ok(("two", "3")))]
    fn part_2_find_digits(
        #[case] input: &str,
        #[case] expected_output: Result<(&str, &str), DayError>,
    ) {
        assert_eq!(find_digits(input).unwrap(), expected_output.unwrap())
    }
}
