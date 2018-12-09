use day04::common;

fn calculate_most_asleep2(logs: Vec<common::Log>) -> Result<u32, common::Error> {
    let schedule = try!(common::gather_schedule(logs));
    let (guard, minute, _) = schedule.iter().fold((0, 0, 0), |data, (current_guard, (_, schedule))| {
        schedule.iter().fold(data, |(guard, minute, count), (current_minute, current_count)| {
            if *current_count > count {
                (*current_guard, *current_minute, *current_count)
            } else {
                (guard, minute, count)
            }
        })
    });
    Ok(guard * minute)
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let data = try!(common::get_data(session));
    Ok(format!("most asleep 2 (ID x Minute): {}", try!(calculate_most_asleep2(data))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: Vec<&str>, expected: u32) {
        let result = calculate_most_asleep2(common::parse_values(input.into_iter()).expect("failed")).expect("failed");
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
        ], 4455);
    }
}
