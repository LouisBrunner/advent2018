use day08::common;

fn calculate_license(graph: &common::Graph) -> u32 {
    graph.children.iter().fold(0, |acc, node| {
        acc + calculate_license(node)
    }) + graph.metadata.iter().sum::<u32>()
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("license: {}", calculate_license(&data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: &str, expected: u32) {
        let result = calculate_license(&common::parse_values(input.split(" ")).expect("failed"));
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 138);
    }
}
