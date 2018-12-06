use core;
use day01::common;

fn calculate_frequency(changes: std::vec::Vec<i32>) -> Result<i32, common::Error>
{
    Ok(changes.iter().fold(0, |acc, change| {
        acc + change
    }))
}

fn solve_problem(session: &str) -> Result<i32, common::Error> {
    let data = try!(common::get_data(session));
    calculate_frequency(data)
}

pub fn solve(session: &str) -> Result<String, core::Error> {
    solve_problem(session).map(|frequency| format!("frequency: {}", frequency))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: i32) {
        let result = calculate_frequency(
            common::parse_values(
                input.into_iter()
            ).expect("failed")
        ).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec!["+1", "+1", "+1"], 3);
    }

    #[test]
    fn example2_works() {
        do_test(vec!["+1", "+1", "-2"], 0);
    }

    #[test]
    fn example3_works() {
        do_test(vec!["-1", "-2", "-3"], -6);
    }
}
