#![feature(generic_arg_infer)]
pub mod day1;

pub fn solution(day: u8) {
    match day {
        1 => {
            day1::solution_p1("day1p1_example.txt");
            day1::solution_p1("day1p1.txt");

            day1::solution_p2("day1p2_example.txt");
            day1::solution_p2("day1p1.txt");
        }
        _ => (),
    }
}
