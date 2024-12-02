use crate::direction::Direction;
use rayon::prelude::*;

pub fn process(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let fails = lines
        .iter()
        .filter_map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("fine"));
            let it = nums.clone().zip(nums.clone().skip(1));

            let mut dir = Direction::Unset;

            let mut still_fails = false;

            for (a, b) in it {
                let abs_dif = a.abs_diff(b);

                if abs_dif > 3 || abs_dif == 0 {
                    still_fails = true;
                    break;
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
                            still_fails = true;
                            break;
                        }
                    }
                    Direction::Increasing => {
                        if diff < 0 {
                            still_fails = true;
                            break;
                        }
                    }
                };
            }

            if !still_fails {
                return None;
            }

            let mex_els = nums.clone().count();

            for i in 0..mex_els {
                let filtered =
                    nums.clone()
                        .enumerate()
                        .filter_map(|(ind, num)| if ind == i { None } else { Some(num) });
                let it = filtered.clone().zip(filtered.skip(1));

                let mut dir = Direction::Unset;
                still_fails = false;

                for (a, b) in it {
                    let abs_dif = a.abs_diff(b);

                    if abs_dif > 3 || abs_dif == 0 {
                        still_fails = true;
                        break;
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
                                still_fails = true;
                                break;
                            }
                        }
                        Direction::Increasing => {
                            if diff < 0 {
                                still_fails = true;
                                break;
                            }
                        }
                    };
                }

                if !still_fails {
                    return None;
                }
            }

            Some(())
        })
        .count();

    (lines.len() - fails).try_into().unwrap()
}

pub fn process_par(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let fails = lines
        .par_iter()
        .filter_map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("fine"));
            let it = nums.clone().zip(nums.clone().skip(1));

            let mut dir = Direction::Unset;

            let mut still_fails = false;

            for (a, b) in it {
                let abs_dif = a.abs_diff(b);

                if abs_dif > 3 || abs_dif == 0 {
                    still_fails = true;
                    break;
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
                            still_fails = true;
                            break;
                        }
                    }
                    Direction::Increasing => {
                        if diff < 0 {
                            still_fails = true;
                            break;
                        }
                    }
                };
            }

            if !still_fails {
                return None;
            }

            let mex_els = nums.clone().count();

            for i in 0..mex_els {
                let filtered =
                    nums.clone()
                        .enumerate()
                        .filter_map(|(ind, num)| if ind == i { None } else { Some(num) });
                let it = filtered.clone().zip(filtered.skip(1));

                let mut dir = Direction::Unset;
                still_fails = false;

                for (a, b) in it {
                    let abs_dif = a.abs_diff(b);

                    if abs_dif > 3 || abs_dif == 0 {
                        still_fails = true;
                        break;
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
                                still_fails = true;
                                break;
                            }
                        }
                        Direction::Increasing => {
                            if diff < 0 {
                                still_fails = true;
                                break;
                            }
                        }
                    };
                }

                if !still_fails {
                    return None;
                }
            }

            Some(())
        })
        .count();

    (lines.len() - fails).try_into().unwrap()
}
