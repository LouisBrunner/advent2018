use core;
use day04::common;

fn calculate_most_asleep(logs: Vec<common::Log>) -> Result<u32, common::Error> {
    let schedule = try!(common::gather_schedule(logs));
    let most_asleep = schedule.iter().max_by_key(|(_, (slept, _))| {
        slept
    });
    match most_asleep {
        None => Err(core::Error::Internal{why: "no most asleep".to_string()}),
        Some((guard, (_, schedule))) => {
            let most_asleep_minute = schedule.iter().max_by_key(|(_, count)| {
                *count
            });
            match most_asleep_minute {
                None => Err(core::Error::Internal{why: "no most asleep minute".to_string()}),
                Some((minute, _)) => Ok(guard * minute),
            }
        }
    }
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("most asleep (ID x Minute): {}", try!(calculate_most_asleep(data))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: u32) {
        let result = calculate_most_asleep(common::parse_values(input.into_iter()).expect("failed")).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ], 240);
    }
}
