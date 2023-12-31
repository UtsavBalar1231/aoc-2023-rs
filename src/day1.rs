// --- Day 1: Trebuchet?! ---
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
//
//
// --- Part Two ---
//
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
//
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?

use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn solution_p1(path: &str) {
        let input_str = std::fs::read_to_string(path).unwrap();
        let mut answer = 0;
        input_str.lines().for_each(|line| {
            let numbers_per_line = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            // println!("{numbers_per_line:?}");

            // Assuming each line has at-least one number
            answer += numbers_per_line.first().unwrap() * 10
                + numbers_per_line
                    .last()
                    .unwrap_or(numbers_per_line.first().unwrap());
        });

        println!("p1: {path} answer: {answer}");
    }

    fn solution_p2(path: &str) {
        let r =
            regex::Regex::new(r"([[:digit:]]|zero|one|two|three|four|five|six|seven|eight|nine)")
                .unwrap();

        let input_str = std::fs::read_to_string(path).unwrap();
        let mut answer = 0;
        input_str.lines().for_each(|line| {
            let mut line = line.to_owned();
            let mut numbers_per_line = Vec::new();

            while !line.is_empty() {
                if let Some(m) = r.find(&line) {
                    numbers_per_line.push(extract_number_from_match(m).unwrap());
                }
                line.drain(..1);
            }
            // Assuming each line has at-least one number
            answer += numbers_per_line.first().unwrap() * 10
                + numbers_per_line
                    .last()
                    .unwrap_or(numbers_per_line.first().unwrap());
        });

        println!("p2: {path} answer: {answer}");
    }
}

fn extract_number_from_match(m: regex::Match<'_>) -> Option<usize> {
    match m.as_str() {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        // Because our regex will only extract [0-9] or their alphabetic version
        _ => m.as_str().parse().ok(),
    }
}
