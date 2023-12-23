fn main() {
    let input = include_str!("../../input.txt");

    let score = input
        .lines()
        .fold(0, |mut acc, line| {
            let opponent  = (line.chars().next().unwrap() as u32)- ('A' as u32) + 1;
            let yours = (line.chars().nth(2).unwrap() as u32) - ('X' as u32) + 1;
            let mut win_score = 0;
            if opponent == yours {
                win_score = 3;
            }
            if (yours == 1 && opponent == 3) || (yours == 2 && opponent == 1) || (yours == 3 && opponent == 2) {
                win_score = 6;
            }

            let score = win_score + yours;

            acc += score;
            acc
        });

    println!("Our score {}", score);

    let score = input
        .lines()
        .fold(0, |mut acc, line| {
            let opponent  = (line.chars().next().unwrap() as u32)- ('A' as u32) + 1;
            let win_score = ((line.chars().nth(2).unwrap() as u32) - ('X' as u32)) * 3;
            let mut score = win_score;
            score += match win_score {
                3 => opponent,
                0 => {
                    match opponent {
                        1 => 3,
                        2 => 1,
                        3 => 2,
                        _ => panic!("Somethings wrong"),
                    }
                }, 
                6 => {
                    match opponent {
                        1 => 2,
                        2 => 3,
                        3 => 1,
                        _ => panic!("Somethings wrong"),
                    }
                },
                _ => panic!("Somethings wrong"),
            };

            acc += score;
            acc
        });

    println!("Our score {}", score);
}
