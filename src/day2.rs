// --- Day 2: Cube Conundrum ---
//
// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
//
// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
//
// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
//
// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
//
// You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
//
// For example, the record of a few games might look like this:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
//
// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
//
// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.
//
// Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
//
// --- Part Two ---
//
// The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!
//
// As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
//
// Again consider the example games from earlier:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
//     In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
//     Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
//     Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
//     Game 4 required at least 14 red, 3 green, and 15 blue cubes.
//     Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
//
// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.
//
// For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?

use crate::utils::Tuple;
use crate::Solution;

pub struct Day2;
const _MAX_RGB: Tuple = Tuple(12, 13, 14);

fn parse_rgb(game_set: &str) -> Tuple {
    let mut rgb = Tuple(0, 0, 0);

    game_set.trim().split(',').for_each(|rgb_color| {
        let rgb_color = rgb_color.split_whitespace().collect::<Vec<_>>();

        match rgb_color[1] {
            "red" => rgb.0 += rgb_color[0].parse::<usize>().unwrap(),
            "green" => rgb.1 += rgb_color[0].parse::<usize>().unwrap(),
            "blue" => rgb.2 += rgb_color[0].parse::<usize>().unwrap(),
            _ => println!("ERROR: unknown color detected {:?}", rgb_color[1]),
        }
    });

    rgb
}

fn get_fewest_cubes(game: &str) -> Tuple {
    let mut rgb = Tuple(0, 0, 0);

    game.trim().split(';').for_each(|game_set| {
        println!("{:?}", game_set);
        game_set.split(',').for_each(|rgb_color| {
            let rgb_color = rgb_color.split_whitespace().collect::<Vec<_>>();

            match rgb_color[1] {
                "red" => {
                    if rgb_color[0].parse::<usize>().unwrap() > rgb.0 {
                        rgb.0 = rgb_color[0].parse::<usize>().unwrap();
                    }
                }
                "green" => {
                    if rgb_color[0].parse::<usize>().unwrap() > rgb.1 {
                        rgb.1 = rgb_color[0].parse::<usize>().unwrap();
                    }
                }
                "blue" => {
                    if rgb_color[0].parse::<usize>().unwrap() > rgb.2 {
                        rgb.2 = rgb_color[0].parse::<usize>().unwrap();
                    }
                }
                _ => println!("ERROR: unknown color detected {:?}", rgb_color[1]),
            }
        });
    });

    rgb
}

fn get_power(rgb: Tuple) -> usize {
    rgb.0 * rgb.1 * rgb.2
}

impl Solution for Day2 {
    fn solution_p1(path: &str) {
        let mut answer = 0;
        'game: for (game_number, game) in std::fs::read_to_string(path)
            .expect("read file failed")
            .lines()
            .enumerate()
        {
            for game_set in game.trim().split(':').nth(1).unwrap().trim().split(';') {
                let rgb = parse_rgb(game_set);

                if !(rgb.0 <= _MAX_RGB.0 && rgb.1 <= _MAX_RGB.1 && rgb.2 <= _MAX_RGB.2) {
                    // println!(
                    //     "{game_number}: valid {rgb:?}",
                    //     game_number = game_number + 1,
                    // );
                    continue 'game;
                }

                // println!("{game_number}: {game} -> {rgb:?}",)
            }
            answer += game_number + 1;
        }

        println!("p1: {path} answer: {answer}",);
    }

    fn solution_p2(path: &str) {
        let mut answer = 0;
        std::fs::read_to_string(path)
            .expect("read file failed")
            .lines()
            .for_each(|game| {
                game.trim().split(':').skip(1).for_each(|game| {
                    let rgb = get_fewest_cubes(game);
                    answer += get_power(rgb);
                });
            });

        println!("p1: {path} answer: {answer}",);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let game = "8 red, 6 blue; 4 blue, 19 red; 4 blue, 9 red; 9 blue, 10 red; 2 green, 9 blue, 13 red; 3 blue, 7 red";

        let mut rgb = Tuple::default();
        game.trim().split(';').for_each(|game| {
            rgb += parse_rgb(game);
        });

        assert_eq!(rgb.0, 66);
        assert_eq!(rgb.1, 2);
        assert_eq!(rgb.2, 35);
    }

    #[test]
    fn test_p1_2() {
        let game = "Game 41: 13 blue, 3 red, 1 green; 2 green, 10 red; 1 blue, 5 red, 3 green; 5 green, 16 blue; 9 blue, 2 green; 14 blue, 4 green, 5 red";

        let mut rgb = Tuple::default();
        game.split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(';')
            .for_each(|game| {
                rgb += parse_rgb(game);
            });

        assert_eq!(rgb.0, 23);
        assert_eq!(rgb.1, 17);
        assert_eq!(rgb.2, 53);
    }

    #[test]
    fn test_p1_3() {
        let game = "Game 88: 5 green, 4 red, 1 blue; 3 blue, 8 red, 10 green; 11 green, 7 red, 4 blue; 11 green, 5 blue, 4 red; 9 red, 9 green; 4 blue, 6 green, 9 red";

        let mut rgb = Tuple::default();

        game.split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(';')
            .for_each(|game| {
                rgb += parse_rgb(game);
            });

        assert_eq!(rgb.0, 41);
        assert_eq!(rgb.1, 52);
        assert_eq!(rgb.2, 17);
    }
}
