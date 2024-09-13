use chrono::Datelike;

pub fn is_christmas_on_sunday(year: u32) -> bool {
    // 1. Get the day of the week for December 25th
    if let Some(day) = chrono::NaiveDate::from_ymd_opt(year as i32, 12, 25) {
        // 2. Check if it's Sunday
        return day.weekday() == chrono::Weekday::Sun;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_christmas_on_sunday() {
        assert_eq!(is_christmas_on_sunday(2021), false);
        assert_eq!(is_christmas_on_sunday(2022), true);
    }
}

//  The  is_christmas_on_sunday  function takes a year as an argument and returns a boolean value indicating whether Christmas falls on a Sunday in that year.
//  The function uses the  chrono  crate to get the day of the week for December 25th of the given year. It then checks if the day of the week is Sunday and returns the result.
//  The  tests  module contains a unit test for the  is_christmas_on_sunday  function.
//  Step 3: Run the Tests
//  To run the tests, use the following command:
//  cargo test

//  The output should indicate that the tests passed:
//  running 1 test
