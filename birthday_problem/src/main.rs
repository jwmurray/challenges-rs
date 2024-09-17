use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
// Create a struct to hold numbers between 1 and 365 inclusive
struct DayNumber(u16);

impl DayNumber {
    fn new(value: u16) -> Option<Self> {
        if value >= 1 && value <= 365 {
            Some(DayNumber(value))
        } else {
            None
        }
    }
}

fn main() {
    let trials = 20_000;
    let mut start_population_size = 2;

    (2_u16..=5_u16).for_each(|min_common| {
        let (min_count_for_50_percent_success, percentage) =
            get_group_size_for_50_percent_success_for_given_common_birthdays(
                min_common,
                trials,
                start_population_size,
            );
        println!(
            "Probability {} people in a group of {} share a common birthday: ({})",
            min_common, min_count_for_50_percent_success, percentage
        );
        start_population_size = min_count_for_50_percent_success;
    });
}

fn get_group_size_for_50_percent_success_for_given_common_birthdays(
    min_common: u16,
    initial_trials: u32,
    start_population_size: u32,
) -> (u32, f32) {
    let runs = 10;
    let mut trials = initial_trials;
    let mut run_vec: Vec<(u32, f32)> = Vec::new();
    let max_population_size = 366;
    loop {
        println!(
            "Loop: Trials: {trials}, shared: {min_common}",
            trials = trials,
            min_common = min_common
        );
        for _ in 0..runs {
            let result = (start_population_size..max_population_size)
                .find_map(|population_size| {
                    let success_average: f32 = get_estimated_success_for_population_size(
                        population_size,
                        min_common,
                        trials,
                    );
                    if success_average >= 0.5 {
                        Some((population_size, success_average))
                    } else {
                        None
                    }
                })
                .unwrap_or((0, 0.0));
            run_vec.push(result);
        }
        let std_dev = get_standard_deviation(&run_vec);
        println!("Standard deviation: {}", std_dev);
        if trials > 1_000_000 || std_dev < 0.02 {
            break;
        } else {
            trials *= 5;
        }
    }
    if run_vec.len() > 4 {
        let count = run_vec[4].0; // get the median value in case the high or the low is off by 1
        let percentage = run_vec
            .iter()
            .map(|(_, percentage)| percentage)
            .sum::<f32>() as f32
            / run_vec.len() as f32;
        (count, percentage)
    } else {
        (0, 0.0)
    }
}

fn get_standard_deviation(run_vec: &Vec<(u32, f32)>) -> f32 {
    let mean: f32 = run_vec
        .iter()
        .map(|(_, percentage)| percentage)
        .sum::<f32>()
        / run_vec.len() as f32;
    let variance: f32 = run_vec
        .iter()
        .map(|(_, percentage)| (percentage - mean).powf(2.0))
        .sum::<f32>()
        / run_vec.len() as f32;
    variance.sqrt()
}

fn get_estimated_success_for_population_size(
    population_size: u32,
    min_common: u16,
    trials: u32,
) -> f32 {
    // Estimate the probability of at least min_common people having the same birthday in a group of n people.
    // The estimate is based on the number of trials where the condition is met.

    let success_average: f32 = (0_u32..trials)
        .map(|_| check_if_random_birthdays_have_common_birthday(population_size, min_common) as u32)
        .sum::<u32>() as f32
        / trials as f32;
    success_average
}

fn check_if_random_birthdays_have_common_birthday(n: u32, min_common: u16) -> bool {
    // Generate n random birthdays and check if there are at least min_common common birthdays.

    let birthdays = generate_population_of_birthdays(n);
    get_max_common_birthday_count(&birthdays) >= min_common as u32
}

fn generate_population_of_birthdays(n: u32) -> Vec<DayNumber> {
    // Generate a vector of n random DayNumbers
    let birthdays: Vec<DayNumber> = (0..n)
        // could call map with .unwrap() as the random number will be in the correct range, but why?
        // Let's let the code filter out the None values in case someone changes the range.
        .filter_map(|_| DayNumber::new(rand::random::<u16>() % 365 + 1))
        .collect();
    birthdays
}

fn get_max_common_birthday_count(birthdays: &Vec<DayNumber>) -> u32 {
    // A common birthday is one where two or more people have the same birthday.
    //
    // returns the number of people with a common birthday.
    // If more than one common birthday is found,
    // the function should return the number of people with the most common birthday.

    let mut common_birthdays: HashMap<u16, u32> = HashMap::new();

    birthdays.iter().for_each(|birthday| {
        *common_birthdays.entry(birthday.0).or_insert(0) += 1;
    });

    let mut max_common_birthday = 0;

    for (_, count) in common_birthdays {
        if count > max_common_birthday {
            max_common_birthday = count;
        }
    }
    max_common_birthday
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_for_common_birthday() {
        let birthdays: Vec<DayNumber> = vec![];
        assert_eq!(get_max_common_birthday_count(&birthdays), 0);
        let birthdays = vec![DayNumber(1), DayNumber(2), DayNumber(3)];
        assert_eq!(get_max_common_birthday_count(&birthdays), 1);
        let birthdays = vec![DayNumber(1), DayNumber(1), DayNumber(3)];
        assert_eq!(get_max_common_birthday_count(&birthdays), 2);
        let birthdays = vec![
            DayNumber(1),
            DayNumber(1),
            DayNumber(360),
            DayNumber(360),
            DayNumber(360),
        ];
        assert_eq!(get_max_common_birthday_count(&birthdays), 3);
    }

    #[test]
    fn test_generate_birthdays() {
        let birthdays = generate_population_of_birthdays(10);
        assert_eq!(birthdays.len(), 10);
        for birthday in birthdays {
            assert!(birthday.0 >= 1 && birthday.0 <= 365);
        }
    }
}
