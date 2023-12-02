// Day 1: Trebuchet?! ---
//
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?

use regex::Regex;

pub fn solution_p1(input_file: &str) {
    let input_str = std::fs::read_to_string(input_file).unwrap();
    let mut calib_values_per_line = Vec::new();
    input_str.lines().for_each(|line| {
        let mut values_per_line = Vec::new();
        line.chars().for_each(|ch| {
            if ch.is_ascii_digit() {
                // println!("{line} has digit: {ch}");
                values_per_line.push(ch);
            }
        });
        calib_values_per_line.push(values_per_line);
    });

    // println!("{calib_values_per_line:?}");

    let mut answer = 0;
    calib_values_per_line.iter().for_each(|line| {
        if !line.is_empty() {
            if line.len() == 1 {
                answer += format!("{}{}", line[0], line[0]).parse::<usize>().unwrap();
            } else {
                answer += format!("{}{}", line.first().unwrap(), line.last().unwrap())
                    .parse::<usize>()
                    .unwrap();
            }
        }
    });

    println!("p1: {input_file} answer: {answer}");
}

fn extract_number_from_match(m: regex::Match<'_>) -> Option<&str> {
    match m.as_str() {
        "zero" => Some("0"),
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        // Because our regex will only extract [0-9] or their alphabetic version
        _ => Some(m.as_str()),
    }
}

pub fn solution_p2(input_file: &str) {
    let r =
        Regex::new(r"(?<x>[[:digit:]]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let input_str = std::fs::read_to_string(input_file).unwrap();
    let mut calib_values_per_line = Vec::new();
    input_str.lines().for_each(|line| {
        let mut values_per_line = Vec::new();
        r.captures_iter(line).for_each(|m| {
            // println!("m: {m:?}");
            if let Some(first) = m.name("x") {
                values_per_line.push(extract_number_from_match(first).unwrap());
            }
        });
        calib_values_per_line.push(values_per_line);
    });
    // println!("p2: {calib_values_per_line:?}");
    let mut answer = 0;
    let mut count = 0;
    calib_values_per_line.iter().for_each(|line| {
        count += 1;
        if !line.is_empty() {
            if line.len() == 1 {
                // println!("{count} adding: {}{}", line[0], line[0]);
                answer += format!("{}{}", line[0], line[0]).parse::<usize>().unwrap();
            } else {
                // println!(
                //     "{count} adding: {}{}",
                //     line.first().unwrap(),
                //     line.last().unwrap()
                // );
                answer += format!("{}{}", line.first().unwrap(), line.last().unwrap())
                    .parse::<usize>()
                    .unwrap();
            }
        }
    });
    println!("p2: {input_file} answer: {answer}");
}
