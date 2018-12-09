use day08::common;

fn calculate_license2(graph: &common::Graph) -> u32 {
    if graph.children.len() == 0 {
        return graph.metadata.iter().sum()
    }
    graph.metadata.iter().map(|ip| {
        let ip = *ip as usize;
        if ip == 0 || ip > graph.children.len() {
            0
        } else {
            calculate_license2(&graph.children[ip - 1])
        }
    }).sum()
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("license2: {}", calculate_license2(&data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: &str, expected: u32) {
        let result = calculate_license2(&common::parse_values(input.split(" ")).expect("failed"));
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 66);
    }
}
