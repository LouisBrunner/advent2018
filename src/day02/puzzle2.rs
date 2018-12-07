use day02::common;

fn find_matching(ids: Vec<String>) -> Result<String, common::Error>
{
    let (result, _) = ids.iter().enumerate().fold(("".to_string(), std::u32::MAX), |(common, diff), (i, id)| {
        ids.iter().skip(i + 1).fold((common, diff), |(common, diff), other_id| {
            let (current_common, current_diff) = id.chars().zip(other_id.chars()).fold(("".to_string(), 0), |(mut common, acc), (chr1, chr2)| {
                if acc > diff {
                    return (common, acc)
                }
                match chr1 {
                    c if c != chr2 => (common, acc + 1),
                    _ => {
                        common.push(chr1);
                        (common, acc)
                    },
                }
            });

            match current_diff {
                d if d < diff => (current_common, current_diff),
                _ => (common, diff),
            }
        })
    });
    Ok(result)
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    find_matching(data).map(|common_chars| format!("common characters: {}", common_chars))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: &str) {
        let result = find_matching(
            input.iter().map(|x| x.to_string()).collect()
        ).expect("failed");
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn example1_works() {
        do_test(vec![
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz",
        ], "fgij");
    }
}
