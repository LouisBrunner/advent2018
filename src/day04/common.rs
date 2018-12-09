use regex::Regex;
use chrono::prelude::*;
use std::collections::HashMap;

use core;

pub type Error = core::Error;

pub enum Event {
    NewGuard{id: u32},
    WakesUp,
    FallsAsleep,
}

pub struct Log {
    pub time: chrono::DateTime<Utc>,
    pub event: Event,
}

pub fn get_data(session: &str) -> Result<Vec<Log>, core::Error> {
    let content = try!(core::request(session, 4));

    parse_values(content.trim().split("\n"))
}

pub fn parse_values<'a, Iin>(logs: Iin) -> Result<Vec<Log>, core::Error>
where
    Iin: Iterator<Item = &'a str>,
{
    lazy_static! {
        static ref LINE: Regex =
            Regex::new(r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<rest>.+)$")
            .expect("regex failed to compile");
        static ref LABEL_GUARD: Regex = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").expect("regex failed to compile");
    }

    logs
        .map(|log_str| -> Result<Log, core::Error> {
            let captures = try!(LINE.captures(log_str)
                .ok_or(core::Error::Internal{why: format!("regex didn't match: `{}`", log_str)}));

            Ok(Log{
                time: Utc.ymd(
                    try!(captures["year"].parse::<i32>()),
                    try!(captures["month"].parse::<u32>()),
                    try!(captures["day"].parse::<u32>()),
                ).and_hms(
                    try!(captures["hour"].parse::<u32>()),
                    try!(captures["minute"].parse::<u32>()),
                    0,
                ),
                event: match &captures["rest"] {
                    "falls asleep" => Event::FallsAsleep,
                    "wakes up" => Event::WakesUp,
                    rest => match LABEL_GUARD.captures(rest) {
                        Some(captures) => Event::NewGuard{id: try!(captures["id"].parse::<u32>())},
                        None => { return Err(core::Error::Internal{why: format!("no match for rest: `{}`", rest)}) ; },
                    },
                },
            })
        })
        .collect::<Result<Vec<Log>, core::Error>>().map(|mut x| {
            x.sort_by(|a, b| a.time.cmp(&b.time));
            x
        })
}

pub fn gather_schedule(logs: Vec<Log>) -> Result<HashMap<u32, (u32, HashMap<u32, u32>)>, core::Error> {
    let mut guards_schedule = HashMap::new();
    let mut current_guard = None;
    let mut current_sleeping = None;
    for (i, log) in logs.iter().enumerate() {
        match log.event {
            Event::FallsAsleep => {
                current_sleeping = Some(log.time);
            },
            Event::WakesUp => {
                let guard = try!(current_guard.ok_or(core::Error::Internal{why: format!("{}: no guard in the schedule", i)}));
                let start_time = try!(current_sleeping.ok_or(core::Error::Internal{why: format!("{}: guard is not sleeping", i)}));

                let end_time = log.time;
                if end_time.hour() > 1 {
                    try!(current_sleeping.ok_or(core::Error::Internal{why: format!("{}: end of sleep is too late", i)}));
                }

                let minute_slept = (end_time - start_time).num_minutes() as u32;
                let start_time = if start_time.hour() > 0 { end_time.date().and_hms(0, 0, 0) } else { start_time };
                if (end_time - start_time).num_minutes() > 0 {
                    try!(current_sleeping.ok_or(core::Error::Internal{why: format!("{}: waking up before sleeping", i)}));
                }

                guards_schedule
                    .entry(guard)
                    .and_modify(|(slept, _): &mut (u32, _)| {
                        *slept += minute_slept;
                    })
                    .or_insert((minute_slept, HashMap::new()));
                for minute in start_time.minute()..end_time.minute() {
                    guards_schedule.entry(guard)
                        .and_modify(|(_, schedule): &mut (_, HashMap<u32, u32>)| {
                            schedule
                                .entry(minute)
                                .and_modify(|count| *count += 1)
                                .or_insert(1);
                        });
                }
                current_sleeping = None;
            },
            Event::NewGuard{id} => {
                current_guard = Some(id);
                current_sleeping = None;
            },
        }
    }
    Ok(guards_schedule)
}
