use crate::direction::Direction;

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let it = line.split_ascii_whitespace();
            let it = it.clone().zip(it.skip(1));

            let mut dir = Direction::Unset;

            for (a, b) in it {
                let a = a.parse::<i32>().expect("Should be a number");
                let b = b.parse::<i32>().expect("Should be a number");

                let abs_dif = a.abs_diff(b);

                if abs_dif > 3 || abs_dif == 0 {
                    return None;
                }

                let diff = a - b;

                match dir {
                    Direction::Unset => {
                        dir = if diff > 0 {
                            Direction::Increasing
                        } else {
                            Direction::Decreasing
                        };
                    }
                    Direction::Decreasing => {
                        if diff > 0 {
                            return None;
                        }
                    }
                    Direction::Increasing => {
                        if diff < 0 {
                            return None;
                        }
                    }
                };
            }

            Some(())
        })
        .count()
        .try_into()
        .unwrap()
}
