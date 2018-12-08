use regex::Regex;
use std::collections::HashMap;

use core;

pub type Error = core::Error;

pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn get_data(session: &str) -> Result<Vec<Claim>, core::Error> {
    let content = try!(core::request(session, 3));

    parse_values(content.trim().split("\n"))
}

pub fn parse_values<'a, Iin>(claims: Iin) -> Result<Vec<Claim>, core::Error>
where
    Iin: Iterator<Item = &'a str>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$").expect("regex failed to compile");
    }

    claims
        .map(|claim_str| -> Result<Claim, core::Error> {
            let captures = try!(RE.captures(claim_str)
                .ok_or(core::Error::Internal{why: format!("regex didn't match: `{}`", claim_str)}));

            Ok(Claim{
                id: try!(captures["id"].parse::<u32>()),
                x: try!(captures["x"].parse::<u32>()),
                y: try!(captures["y"].parse::<u32>()),
                width: try!(captures["width"].parse::<u32>()),
                height: try!(captures["height"].parse::<u32>()),
            })
        })
        .collect::<Result<Vec<Claim>, core::Error>>()
}

pub fn create_map(claims: Vec<Claim>) -> HashMap<(u32, u32), Vec<u32>> {
    let mut map = HashMap::new();
    for claim in claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                let coords = (claim.x + i, claim.y + j);
                map.entry(coords).and_modify(|x: &mut Vec<u32>| x.push(claim.id)).or_insert(vec![claim.id]);
            }
        }
    }
    map
}
