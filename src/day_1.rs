use crate::day::{Day, DayError};

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
        Ok(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet";

    #[test]
    fn part_1() {
        assert_eq!(Day1.part_1(INPUT).unwrap(), "142");
    }
}
