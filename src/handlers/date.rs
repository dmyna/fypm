use chrono::{Duration, NaiveDate};

pub struct NaiveDateIter {
    start: NaiveDate,
    end: NaiveDate,
}
impl NaiveDateIter {
    pub fn new(start: NaiveDate, end: NaiveDate) -> NaiveDateIter {
        NaiveDateIter { start, end }
    }
}
impl Iterator for NaiveDateIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let next = self.start;
            self.start = self.start.checked_add_signed(Duration::days(1)).unwrap();
            Some(next)
        } else {
            None
        }
    }
}
