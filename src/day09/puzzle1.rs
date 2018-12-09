use std::collections::HashMap;

use core;
use day09::common;

fn increment_index(index: usize, change: i32, circle: &Vec<u32>) -> usize {
    let length = circle.len();
    let mut new_index = index as i32 + change;
    if new_index < 0 {
        new_index += length as i32;
    }
    let mut new_index = new_index as usize;
    if new_index > length {
       new_index %= length;
    }
    new_index
}

fn calculate_score(players: u32, worth: u32) -> Result<u32, common::Error> {
    let mut circle = Vec::with_capacity(worth as usize);
    circle.push(0);
    let mut index = 1;
    let mut scores = HashMap::new();
    let mut advance = false;
    for marble in 1..=worth {
        let player = marble % players;
        if marble % 23 == 0 {
            let new_index = increment_index(index, -7, &circle);
            let score = scores.entry(player).or_insert(0);
            *score += circle.remove(new_index) + marble;
            index = increment_index(new_index, 1, &circle);
            advance = false;
        } else {
            if advance {
                index = increment_index(index, 2, &circle);
            }
            advance = true;
            circle.insert(index, marble);
        }
        println!("[{}] {:?} -> {:?}]", marble % players, circle, scores);
    }

    scores
        .iter()
        .max_by_key(|(_, score)| *score)
        .map(|(_, score)| *score)
        .ok_or(core::Error::Internal{why: "no highscore".to_string()})
}

pub fn solve(session: &str) -> Result<String, common::Error> {
    let (players, worth) = try!(common::get_data(session));
    Ok(format!("info: {}", try!(calculate_score(players, worth))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input_players: u32, input_worth: u32, expected: u32) {
        let result = calculate_score(input_players, input_worth).expect("failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn example1_works() {
        do_test(9, 25, 32);
    }

    // #[test]
    // fn example2_works() {
    //     do_test(10, 1618, 8317);
    // }
    //
    // #[test]
    // fn example3_works() {
    //     do_test(13, 7999, 146373);
    // }

    #[test]
    fn example4_works() {
        do_test(17, 1104, 2764);
    }

    // #[test]
    // fn example5_works() {
    //     do_test(21, 6111, 54718);
    // }
    //
    // #[test]
    // fn example6_works() {
    //     do_test(30, 5807, 37305);
    // }
}
