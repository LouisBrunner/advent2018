use std::collections::HashMap;

use core;
use day03::common;

fn find_no_overlap(claims: Vec<common::Claim>) -> Result<u32, common::Error> {
    let mut uniques = HashMap::new();
    common::create_map(claims).iter().for_each(|(_coords, ids)| {
        if ids.len() > 1 {
            for id in ids {
                uniques.insert(*id, false);
            }
        } else {
            uniques.entry(ids[0]).or_insert(true);
        }
    });
    uniques.iter().find(|(_id, unique)| {
        **unique
    }).map(|(id, _unique)| *id).ok_or(core::Error::Internal{why: "could not find non-overlapping claim".to_string()})
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    find_no_overlap(data).map(|id| format!("claim with no overlap: {}", id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: u32) {
        let result = find_no_overlap(common::parse_values(input.into_iter()).expect("failed")).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec![
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2",
        ], 3);
    }
}
