pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod utils;

pub fn solution(day: u8) {
    match day {
        1 => {
            day1::Day1::solution_p1("day1p1_example.txt");
            day1::Day1::solution_p1("day1p1.txt");

            day1::Day1::solution_p2("day1p2_example.txt");
            day1::Day1::solution_p2("day1p1.txt");
        }
        2 => {
            day2::Day2::solution_p1("day2p1_example.txt");
            day2::Day2::solution_p1("day2p1.txt");

            day2::Day2::solution_p2("day2p2_example.txt");
            day2::Day2::solution_p2("day2p1.txt");
        }
        3 => {
            day3::Day3::solution_p1("day3p1_example.txt");
            day3::Day3::solution_p1("day3p1.txt");

            day3::Day3::solution_p2("day3p1_example.txt");
            day3::Day3::solution_p2("day3p1.txt");
        }
        4 => {
            day4::Day4::solution_p1("day4p1_example.txt");
            day4::Day4::solution_p1("day4p1.txt");

            day4::Day4::solution_p2("day4p1_example.txt");
            day4::Day4::solution_p2("day4p1.txt");
        }
        _ => (),
    }
}

pub trait Solution {
    fn solution_p1(path: &str);
    fn solution_p2(path: &str);
}
