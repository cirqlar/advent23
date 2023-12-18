fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> isize {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut polys = Vec::new();

    input.lines().for_each(|line| {
        let mut it = line.split_ascii_whitespace();
        it.next();
        it.next();

        //(#70c710)
        let hex = it.next().unwrap();
        let count_as_hex = &hex[2..7];
        let count = isize::from_str_radix(count_as_hex, 16).expect("parseable");
        let dir = &hex[7..8];

        match dir {
            "0" => {
                current_x += count;
            }
            "2" => {
                current_x -= count;
            }
            "3" => {
                current_y -= count;
            }
            "1" => {
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
    fn d18_part_2_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 952408144115);
    }
}
