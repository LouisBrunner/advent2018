use day03::common;

fn calculate_overlap(claims: Vec<common::Claim>) -> u32 {
    common::create_map(claims).iter().filter(|(_coords, ids)| {
        ids.len() > 1
    }).count() as u32
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("overlap (sq inches): {}", calculate_overlap(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: u32) {
        let result = calculate_overlap(common::parse_values(input.into_iter()).expect("failed"));
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec![
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2",
        ], 4);
    }
}
