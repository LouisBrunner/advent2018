use regex::Regex;

use core;

pub type Error = core::Error;

pub fn get_data(session: &str) -> Result<(u32, u32), core::Error> {
    let content = try!(core::request(session, 9));

    parse_values(content.trim())
}

pub fn parse_values<'a>(input: &'a str) -> Result<(u32, u32), core::Error>
{
    lazy_static! {
        static ref INPUT: Regex = Regex::new(r"^(?P<players>\d+) players; last marble is worth (?P<worth>\d+) points$").expect("regex failed to compile");
    }

    let captures = try!(INPUT.captures(input)
        .ok_or(core::Error::Internal{why: format!("regex didn't match: `{}`", input)}));

    Ok((
        try!(captures["players"].parse::<u32>()),
        try!(captures["worth"].parse::<u32>()),
    ))
}
