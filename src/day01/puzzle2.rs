use std::collections::HashMap;
use std::iter::FromIterator;

use core;
use day01::common;

fn find_frequency(changes: std::vec::Vec<i32>) -> Result<i32, common::Error>
{
    let mut found = HashMap::new();
    found.insert(0, true);
    let mut frequency = 0;
    let changes = Vec::from_iter(changes);
    loop {
        for change in changes.iter() {
            frequency += change;
            match found.get(&frequency) {
                Some(_) => {
                    return Ok(frequency);
                },
                None => {
                    found.insert(frequency, true);
                },
            };
        }
    }
}

fn solve_problem(session: &str) -> Result<i32, common::Error> {
    let data = try!(common::get_data(session));
    find_frequency(data)
}

pub fn solve(session: &str) -> Result<String, core::Error> {
    solve_problem(session).map(|frequency| format!("frequency: {}", frequency))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: i32) {
        let result = find_frequency(
            common::parse_values(
                input.into_iter()
            ).expect("failed")
        ).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec!["+1", "-1"], 0);
    }

    #[test]
    fn example2_works() {
        do_test(vec!["+3", "+3", "+4", "-2", "-4"], 10);
    }

    #[test]
    fn example3_works() {
        do_test(vec!["-6", "+3", "+8", "+5", "-6"], 5);
    }

    #[test]
    fn example4_works() {
        do_test(vec!["+7", "+7", "-2", "-7", "-4"], 14);
    }
}
