// --- Day 3: Gear Ratios ---
//
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
//
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//
// "Aaah!"
//
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
//
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
//
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
//
// --- Part Two ---
//
// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
//
// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
//
// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
//
// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
//
// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
//
// Consider the same engine schematic again:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//
// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
//
// What is the sum of all of the gear ratios in your engine schematic?

use regex::Regex;

use crate::Solution;

pub struct Day3;

#[derive(Debug, Default)]
struct Number {
    value: usize,
    x: isize,
    y: isize,
    valid: bool,
}

#[derive(Debug)]
struct Symbol {
    x: isize,
    y: isize,
    c: char,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}): {}", self.x, self.y, self.value)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Solution for Day3 {
    fn solution_p1(path: &str) {
        let input = std::fs::read_to_string(path).unwrap();

        let mut answer = 0;
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();
        let r = Regex::new(r"(\d+)").unwrap();
        input.lines().enumerate().for_each(|(y, line)| {
            numbers.push(Number::default());

            line.chars().enumerate().for_each(|(x, c)| {
                if matches!(c, '#' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@' | '$') {
                    symbols.push(Symbol {
                        x: x as isize,
                        y: y as isize,
                        c,
                    });
                }
                if let Some(m) = r.find(&c.to_string()) {
                    if c.is_ascii_digit() {
                        if let Some(n) = numbers.last_mut() {
                            // println!("Modifying number: {} to {}{}", n.value, n.value, m.as_str());
                            n.value = n.value * 10 + m.as_str().parse::<usize>().unwrap();
                            n.x = x as isize;
                            n.y = y as isize;
                        }
                        // If the next char is not a digit, but there still exists
                        // another number ahead, push a new number
                        if x < line.len() - 1
                            && !line.chars().nth(x + 1).unwrap().is_ascii_digit()
                            && line.chars().skip(x + 1).any(|c| c.is_ascii_digit())
                        {
                            numbers.push(Number::default());
                        }
                    }
                }
            });
        });
        // println!("Numbers: {:#?}, count: {}", numbers, numbers.len());
        numbers.iter().for_each(|n| {
            symbols.iter().for_each(|s| {
                if ((s.x - n.x).abs() <= 1
                    || (s.x - (n.x - (n.value.to_string().len() as isize - 1))).abs() <= 1)
                    && (s.y - n.y).abs() <= 1
                    && !n.valid
                    && n.value > 0
                {
                    // println!("Found: {}", n);
                    answer += n.value;
                }
            });
        });

        println!("p1: {path} answer: {answer}",);
    }

    fn solution_p2(path: &str) {
        let mut answer = 0;
        let input = std::fs::read_to_string(path).unwrap();
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();
        let r = Regex::new(r"(\d+)").unwrap();
        input.lines().enumerate().for_each(|(y, line)| {
            numbers.push(Number::default());

            line.chars().enumerate().for_each(|(x, c)| {
                if matches!(c, '#' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@' | '$') {
                    symbols.push(Symbol {
                        x: x as isize,
                        y: y as isize,
                        c,
                    });
                }
                if let Some(m) = r.find(&c.to_string()) {
                    if c.is_ascii_digit() {
                        if let Some(n) = numbers.last_mut() {
                            // println!("Modifying number: {} to {}{}", n.value, n.value, m.as_str());
                            n.value = n.value * 10 + m.as_str().parse::<usize>().unwrap();
                            n.x = x as isize;
                            n.y = y as isize;
                        }
                        // If the next char is not a digit, but there still exists
                        // another number ahead, push a new number
                        if x < line.len() - 1
                            && !line.chars().nth(x + 1).unwrap().is_ascii_digit()
                            && line.chars().skip(x + 1).any(|c| c.is_ascii_digit())
                        {
                            numbers.push(Number::default());
                        }
                    }
                }
            });
        });

        'symbol: for s in symbols.iter() {
            if s.c == '*' {
                for (idx, n) in numbers.iter().enumerate() {
                    if ((s.x - n.x).abs() <= 1
                        || (s.x - (n.x - (n.value.to_string().len() as isize - 1))).abs() <= 1)
                        && (s.y - n.y).abs() <= 1
                        && n.value > 0
                    {
                        // println!("Found the first: {} at idx: {}", n, idx);

                        for (_idx, v) in numbers.iter().enumerate().skip(idx + 1) {
                            // println!("searching at idx: {}, next: {} for symbol: {}", idx, v, s);

                            if ((s.x - v.x).abs() <= 1
                                || (s.x - (v.x - (v.value.to_string().len() as isize - 1))).abs()
                                    <= 1)
                                && (s.y - v.y).abs() <= 1
                                && v.value > 0
                            {
                                // println!("Found next: {}", v);
                                answer += n.value * v.value;
                                continue 'symbol;
                            }
                        }
                    }
                }
            }
        }

        println!("p2: {path} answer: {answer}",);
    }
}
