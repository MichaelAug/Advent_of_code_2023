use crate::day::{Day, DayError};
use std::str::FromStr;

static RED: u32 = 12;
static GREEN: u32 = 13;
static BLUE: u32 = 14;

pub struct Day2;

#[derive(Default, Debug)]
struct Round {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = DayError;

    // Returns a Round with max values for each colour
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Round::default();

        for part in s.split([',', ':', ';']) {
            let num = part
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()?;

            if part.contains("Game") {
                r.id = num;
            } else if part.contains("red") && (num > r.red) {
                r.red = num;
            } else if part.contains("green") && (num > r.green) {
                r.green = num;
            } else if part.contains("blue") && (num > r.blue) {
                r.blue = num;
            }
        }
        Ok(r)
    }
}

fn get_rounds(input: &str) -> Vec<Round> {
    input
        .lines()
        .filter_map(|l| l.parse::<Round>().ok())
        .collect()
}

impl Day for Day2 {
    fn part_1(&self, input: &str) -> Result<String, DayError> {
        Ok(get_rounds(input)
            .iter()
            .filter(|&r| r.red <= RED && r.green <= GREEN && r.blue <= BLUE)
            .fold(0, |acc, r| acc + r.id)
            .to_string())
    }

    fn part_2(&self, input: &str) -> Result<String, DayError> {
        Ok(get_rounds(input)
            .iter()
            .fold(0, |acc, r| acc + r.red * r.green * r.blue)
            .to_string())
    }
}
