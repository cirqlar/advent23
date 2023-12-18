fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> i32 {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut polys = Vec::new();

    input.lines().for_each(|line| {
        let mut it = line.split_ascii_whitespace();
        let dir = it.next().unwrap();
        let count = it.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "R" => {
                current_x += count;
            }
            "L" => {
                current_x -= count;
            }
            "U" => {
                current_y -= count;
            }
            "D" => {
                current_y += count;
            }
            _ => unreachable!(),
        }
        polys.push((current_x, current_y));
    });

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if xs.contains(&x) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut area = 0;
    let mut perimiter = 0;
    for i in 0..polys.len() {
        let (x0, y0) = polys[i];
        let (x1, y1) = polys[(i + 1) % polys.len()];
        area += x0 * y1 - x1 * y0;

        perimiter += (x1 - x0).abs() + (y1 - y0).abs()
    }
    (area.abs() / 2) + (perimiter / 2) + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d18_part_1_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 62);
    }
}
