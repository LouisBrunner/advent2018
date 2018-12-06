use core;

pub type Error = core::Error;

pub fn get_data(session: &str) -> Result<std::vec::Vec<i32>, core::Error>
{
    let content = try!(core::request(session, 1));

    parse_values(content.trim().split("\n"))
}

pub fn parse_values<'a, Iin>(changes: Iin) -> Result<std::vec::Vec<i32>, core::Error>
where
    Iin: Iterator<Item = &'a str>,
{
    changes.map(|change| -> Result<i32, core::Error> {
        if change.len() < 1 {
            return Err(core::Error::Internal{why: format!("empty frequency change")})
        }
        let frequency_change = try!(change[1..].parse::<i32>());
        if change.starts_with('+') {
            Ok(frequency_change)
        } else if change.starts_with('-') {
            Ok(-frequency_change)
        } else {
            Err(core::Error::Internal{why: format!("invalid frequency change: `{}`", change)})
        }
    }).collect::<Result<std::vec::Vec<i32>, core::Error>>()
}
