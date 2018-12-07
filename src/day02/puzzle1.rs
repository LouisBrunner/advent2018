use std::collections::HashMap;

use day02::common;

fn calculate_checksum(ids: Vec<String>) -> Result<i32, common::Error>
{
    let (doubles, triples) = ids.iter().fold((0, 0), |(doubles, triples), id| {
        let mut occurences = HashMap::new();
        for chr in id.chars() {
            occurences.entry(chr)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let (double, triple) = occurences.iter().fold((false, false), |(double, triple), (_chr, count)| {
            match count {
                2 => (true, triple),
                3 => (double, true),
                _ => (double, triple),
            }
        });
        (doubles + if double { 1 } else { 0 }, triples + if triple { 1 } else { 0 })
    });
    Ok(doubles * triples)
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    calculate_checksum(data).map(|checksum| format!("checksum: {}", checksum))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: i32) {
        let result = calculate_checksum(
            input.iter().map(|x| x.to_string()).collect()
        ).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ], 12);
    }
}
