use day01::common;

fn calculate_frequency(changes: Vec<i32>) -> i32 {
    changes.iter().sum()
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("frequency: {}", calculate_frequency(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: i32) {
        let result = calculate_frequency(common::parse_values(input.into_iter()).expect("failed"));
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
