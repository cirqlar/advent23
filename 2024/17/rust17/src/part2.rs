use crate::get_reg_operand_a;

fn get_combo_operand(operand: usize, a_reg: usize, b_reg: usize, c_reg: usize) -> usize {
    match operand {
        x if x <= 3 => x,
        4 => a_reg,
        5 => b_reg,
        6 => c_reg,
        x => panic!("Invalid combo operand {:?}", x),
    }
}

fn run_program(mut a_reg: usize, program: &[usize]) -> usize {
    let mut b_reg = 0;
    let mut c_reg = 0;

    let mut prog_counter = 0;

    while prog_counter < program.len() {
        let op_code = program[prog_counter];
        let operand = program[prog_counter + 1];

        match op_code {
            0 => {
                a_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            1 => b_reg ^= operand,
            2 => b_reg = get_combo_operand(operand, a_reg, b_reg, c_reg) % 8,
            3 => {
                // Not necessary since there are no jumps before out in our input but it's possible
                if a_reg != 0 {
                    prog_counter = operand;
                    continue;
                }
            }
            4 => b_reg ^= c_reg,
            5 => {
                return get_combo_operand(operand, a_reg, b_reg, c_reg) % 8;
            }
            6 => {
                b_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            7 => {
                c_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            x => unreachable!("Invalid op_code {:?}", x),
        }

        prog_counter += 2;
    }

    unreachable!("We should not be reaching here, there must always be an out");
}

pub fn process(input: &[u8]) -> usize {
    let mut interator = input
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty());

    let _a_reg = interator.next();
    let _b_reg = interator.next();
    let _c_reg = interator.next();

    let program = interator.next().expect("program")[9..]
        .iter()
        .filter(|ch| ch != &&b',')
        .map(get_reg_operand_a)
        .collect::<Vec<_>>();

    let adv_position = program
        .iter()
        .enumerate()
        .step_by(2)
        .find_map(|(i, n)| (n == &0).then_some(i))
        .unwrap();
    let adv_operand = program[adv_position + 1];
    let our_divisor = 2_usize.pow(adv_operand.try_into().unwrap());

    // On the last loop, A must be zero so the last jump doesn't happen and the program halts
    let mut possible_values_of_a = vec![0];
    for current_output_index in (0..program.len()).rev() {
        let mut new_possibilites = vec![];

        for value_of_a in possible_values_of_a.iter() {
            let first_val = value_of_a * our_divisor;
            for i in 0..our_divisor {
                new_possibilites.push(first_val + i);
            }
        }

        possible_values_of_a.clear();

        for new_a in new_possibilites.into_iter().filter(|a| a != &0) {
            let out_value = run_program(new_a, &program);

            if out_value == program[current_output_index] {
                possible_values_of_a.push(new_a);
            }
        }
    }

    possible_values_of_a.into_iter().min().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part2_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 117440);
    }
}
