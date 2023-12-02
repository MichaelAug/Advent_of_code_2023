use thiserror::Error;

pub trait Day {
    fn part_1(&self, input: &str) -> Result<String, DayError>;
    fn part_2(&self, input: &str) -> Result<String, DayError>;

    fn solve(&self, input: &str) -> Result<String, DayError> {
        let p1 = self.part_1(input)?;
        let p2 = self.part_2(input)?;

        Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
    }
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Input error: {0}")]
    Input(String),
}
