use core;
use day09::common;

// fn CODE_LOGIC() -> Result<SOMETHING, common::Error> {
//     TODO
// }

pub fn solve(_session: &str) -> Result<String, common::Error> {
    // let data = try!(common::get_data(session));
    // CODE_LOGIC(data).map(|info| format!("info: {}", info))
    Err(core::Error::Internal{why: "not implemented".to_string()})
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn do_test(input: Vec<&str>, expected: u32) {
    //     let result = CODE_LOGIC(common::parse_values(input.into_iter()).expect("failed")).expect("failed");
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn example1_works() {
        // do_test(vec![
        //     data,
        // ], expected);
    }
}
