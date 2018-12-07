use core;

pub type Error = core::Error;

pub fn get_data(session: &str) -> Result<Vec<String>, core::Error>
{
    let content = try!(core::request(session, 2));

    Ok(content.trim().split("\n").map(|x| x.to_string()).collect::<Vec<String>>())
}
