use std::collections::HashMap;

use core;

fn calculate_checksum(ids: Vec<String>) -> i32 {
    let (doubles, triples) = ids.iter().fold((0, 0), |(doubles, triples), id| {
        let mut occurences = HashMap::new();
        for chr in id.chars() {
            occurences.entry(chr).and_modify(|x| *x += 1).or_insert(1);
        }
        let (double, triple) = occurences.iter().fold(
            (false, false),
            |(double, triple), (_chr, count)| match count {
                2 => (true, triple),
                3 => (double, true),
                _ => (double, triple),
            },
        );
        (
            doubles + if double { 1 } else { 0 },
            triples + if triple { 1 } else { 0 },
        )
    });
    doubles * triples
}

pub fn solve(session: &str) -> Result<String, core::Error> {
    let data = try!(core::get_data_as_strings(session, 2));
    Ok(format!("checksum: {}", calculate_checksum(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: i32) {
        let result = calculate_checksum(input.iter().map(|x| x.to_string()).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(
            vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
            ],
            12,
        );
    }
}
