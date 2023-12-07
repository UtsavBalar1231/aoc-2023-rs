pub mod day1;
pub mod day2;
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
        _ => (),
    }
}

pub trait Solution {
    fn solution_p1(path: &str);
    fn solution_p2(path: &str);
}
