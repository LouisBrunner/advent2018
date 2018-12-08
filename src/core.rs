use custom_error::custom_error;
use reqwest::header::COOKIE;
use reqwest::Client;

custom_error!{pub Error
    Network{source: reqwest::Error} = "network error",
    IO{source: std::io::Error}      = "IO error",
    Parse{source: std::num::ParseIntError}      = "parse error",
    Internal{why: String}        = "error: {why}"
}

pub fn request(session: &str, day: i32) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let req = client
        .get(format!("https://adventofcode.com/2018/day/{}/input", day).as_str())
        .header(COOKIE, format!("session={}", session));

    let mut res = try!(req.send());

    Ok(try!(res.text()))
}

pub fn get_data_as_strings(session: &str, day: i32) -> Result<Vec<String>, Error> {
    let content = try!(request(session, day));

    Ok(content
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>())
}
