use std::str::from_utf8;

enum Stage {
    FindingMuls,
    FirstNum,
    SecondNum,
}

fn find_byte(input: &[u8], byte: &u8) -> Option<usize> {
    for (ind, el) in input.iter().enumerate() {
        if el == byte {
            return Some(ind);
        }
    }

    None
}

pub fn process(input: &[u8]) -> i32 {
    let mut position = 0;
    let mut stage = Stage::FindingMuls;

    let mut total = 0;
    let mut first_num = 0;

    while position < input.len() {
        match stage {
            Stage::FindingMuls => match input[position] {
                b'm' => {
                    if input[position..(position + 4)] == *b"mul(" {
                        stage = Stage::FirstNum;
                        position += 4;
                        continue;
                    } else {
                        position += 1;
                        continue;
                    }
                }
                _ => {
                    position += 1;
                    continue;
                }
            },
            Stage::FirstNum => {
                if input[position].is_ascii_digit() {
                    if let Some(comma_position) = find_byte(&input[position..], &b',') {
                        if let Ok(parsed_str) =
                            from_utf8(&input[position..(position + comma_position)])
                        {
                            if let Ok(parsed_num) = parsed_str.parse::<i32>() {
                                first_num = parsed_num;
                                position += comma_position + 1;
                                stage = Stage::SecondNum;
                                continue;
                            }
                        }

                        stage = Stage::FindingMuls;
                        position += 1;
                        continue;
                    } else {
                        // Todo: potentially just quit the whole thing because no more commas means no more valid muls
                        // Turns out its slower to break here lol. Fancy

                        stage = Stage::FindingMuls;
                        position += 1;
                        continue;
                    }
                } else {
                    stage = Stage::FindingMuls;
                    continue;
                }
            }
            Stage::SecondNum => {
                if input[position].is_ascii_digit() {
                    if let Some(closing_position) = find_byte(&input[position..], &b')') {
                        if let Ok(parsed_str) =
                            from_utf8(&input[position..(position + closing_position)])
                        {
                            if let Ok(parsed_num) = parsed_str.parse::<i32>() {
                                total += first_num * parsed_num;
                                position += closing_position + 1;
                                stage = Stage::FindingMuls;
                                continue;
                            }
                        }

                        stage = Stage::FindingMuls;
                        position += 1;
                        continue;
                    } else {
                        // Todo: potentially just quit the whole thing because no more commas means no more valid muls
                        // Turns out its slower to break here lol. Fancy

                        stage = Stage::FindingMuls;
                        position += 1;
                        continue;
                    }
                } else {
                    stage = Stage::FindingMuls;
                    continue;
                }
            }
        }
    }

    total
}
