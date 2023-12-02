use std::cmp::max;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 12, 13, 14);

    println!("Answer is {}", answer);
}

fn process(input: &str, max_red: i32, max_green: i32, max_blue: i32) -> i32 {
    let mut valid = 0;

    for l in input.lines() {
        let mut split = l.split(':');
        let game = split.next().expect("should have game");
        let rest = split.next().expect("should have tries");

        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        rest.split(';')
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
            .map(|s| {
                let mut n = s.split(' ');
                (
                    n.next().expect("should have beginning"),
                    n.next().expect("should have end"),
                )
            })
            .for_each(|t| {
                let num = t.0.parse::<i32>().expect("should be a number");
                match t.1 {
                    "blue" => blue = max(blue, num),
                    "red" => red = max(red, num),
                    "green" => green = max(green, num),
                    _ => panic!("Should never reach"),
                }
            });

        if red <= max_red && blue <= max_blue && green <= max_green {
            valid += game.split(' ').collect::<Vec<_>>()[1]
                .parse::<i32>()
                .expect("should have a game");
        }
    }

    valid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d02_part_1_is_correct() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input, 12, 13, 14), 8);
    }
}
