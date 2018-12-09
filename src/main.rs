#[macro_use] extern crate lazy_static;
extern crate custom_error;
extern crate reqwest;
extern crate regex;
extern crate chrono;
extern crate clap;
use clap::{App, Arg};

use std::collections::HashMap;

mod core;
mod day01;
mod day02;
mod day03;
mod day04;
mod day08;
// mod day09;

type Puzzle = fn(&str) -> Result<String, core::Error>;

fn main() {
    let matches = App::new("Advent of Code 2018")
        .author("Louis Brunner <louis.brunner.fr@gmail.com>")
        .arg(
            Arg::with_name("session")
                .short("s")
                .long("session")
                .value_name("SESSION")
                .help("Your session cookie")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Choose the day to execute")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("puzzle")
                .short("p")
                .long("puzzle")
                .value_name("PUZZLE")
                .help("Choose the puzzle of the day to execute (default: 1)")
                .takes_value(true),
        )
        .get_matches();

    let session = matches
        .value_of("session")
        .expect("no session was provided");
    let day = matches.value_of("day").expect("no day was provided");
    let puzzle = matches.value_of("puzzle").unwrap_or("1");

    let day_puzzle = [
        (
            "1",
            (
                day01::puzzle1::solve as Puzzle,
                day01::puzzle2::solve as Puzzle,
            ),
        ),
        (
            "2",
            (
                day02::puzzle1::solve as Puzzle,
                day02::puzzle2::solve as Puzzle,
            ),
        ),
        (
            "3",
            (
                day03::puzzle1::solve as Puzzle,
                day03::puzzle2::solve as Puzzle,
            ),
        ),
        (
            "4",
            (
                day04::puzzle1::solve as Puzzle,
                day04::puzzle2::solve as Puzzle,
            ),
        ),
        (
            "8",
            (
                day08::puzzle1::solve as Puzzle,
                day08::puzzle2::solve as Puzzle,
            ),
        ),
        // (
        //     "9",
        //     (
        //         day09::puzzle1::solve as Puzzle,
        //         day09::puzzle2::solve as Puzzle,
        //     ),
        // ),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    let res = match day_puzzle.get(day) {
        Some((puzzle1, puzzle2)) => match puzzle {
            "1" => puzzle1(session),
            "2" => puzzle2(session),
            _ => Err(core::Error::Internal {
                why: "unknown puzzle".to_string(),
            }),
        },
        None => Err(core::Error::Internal {
            why: "could not find that day".to_string(),
        }),
    };

    match res {
        Ok(content) => println!("{}", content),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
