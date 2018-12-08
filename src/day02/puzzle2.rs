use core;

fn find_matching(ids: Vec<String>) -> String {
    ids.iter().enumerate().fold(
        ("".to_string(), std::u32::MAX),
        |(common, diff), (i, id)| {
            ids.iter()
                .skip(i + 1)
                .fold((common, diff), |(common, diff), other_id| {
                    let (current_common, current_diff) = id.chars().zip(other_id.chars()).fold(
                        ("".to_string(), 0),
                        |(mut common, acc), (chr1, chr2)| {
                            if acc > diff {
                                return (common, acc);
                            }
                            match chr1 {
                                c if c != chr2 => (common, acc + 1),
                                _ => {
                                    common.push(chr1);
                                    (common, acc)
                                }
                            }
                        },
                    );

                    match current_diff {
                        d if d < diff => (current_common, current_diff),
                        _ => (common, diff),
                    }
                })
        },
    ).0
}

pub fn solve(session: &str) -> Result<String, core::Error> {
    let data = try!(core::get_data_as_strings(session, 2));
    Ok(format!("common characters: {}", find_matching(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: &str) {
        let result = find_matching(input.iter().map(|x| x.to_string()).collect());
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn example1_works() {
        do_test(
            vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ],
            "fgij",
        );
    }
}
