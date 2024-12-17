use std::str::from_utf8;

fn get_combo_operand(operand: &[u8], a_reg: usize, b_reg: usize, c_reg: usize) -> usize {
    match operand {
        b"0" => 0,
        b"1" => 1,
        b"2" => 2,
        b"3" => 3,
        b"4" => a_reg,
        b"5" => b_reg,
        b"6" => c_reg,
        x => panic!("Invalid combo operand {:?}", x),
    }
}

fn get_reg_operand(operand: &[u8]) -> usize {
    match operand {
        b"0" => 0,
        b"1" => 1,
        b"2" => 2,
        b"3" => 3,
        b"4" => 4,
        b"5" => 5,
        b"6" => 6,
        b"7" => 7,
        b"8" => 8,
        b"9" => 9,
        x => panic!("Invalid reg operand {:?}", x),
    }
}

pub fn process(input: &[u8]) -> String {
    let mut interator = input
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty());

    let mut a_reg = from_utf8(&interator.next().expect("a reg")[12..])
        .expect("a reg")
        .parse::<usize>()
        .expect("a reg");
    let mut b_reg = from_utf8(&interator.next().expect("b reg")[12..])
        .expect("b reg")
        .parse::<usize>()
        .expect("b reg");
    let mut c_reg = from_utf8(&interator.next().expect("c reg")[12..])
        .expect("c reg")
        .parse::<usize>()
        .expect("c reg");

    let program = interator.next().expect("program")[9..]
        .split(|ch| ch == &b',')
        .collect::<Vec<_>>();

    let mut prog_counter = 0;
    let mut output = vec![];

    while prog_counter < program.len() {
        let op_code = program[prog_counter];
        let operand = program[prog_counter + 1];

        match op_code {
            b"0" => {
                a_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            b"1" => b_reg ^= get_reg_operand(operand),
            b"2" => b_reg = get_combo_operand(operand, a_reg, b_reg, c_reg) % 8,
            b"3" => {
                if a_reg != 0 {
                    prog_counter = get_reg_operand(operand);
                    continue;
                }
            }
            b"4" => b_reg ^= c_reg,
            b"5" => output.push(get_combo_operand(operand, a_reg, b_reg, c_reg) % 8),
            b"6" => {
                b_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            b"7" => {
                c_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            x => panic!("Invalid op_code {:?}", x),
        }

        prog_counter += 2;
    }

    output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")

    // "0".into()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(&answer, "4,6,3,5,6,3,5,2,1,0");
    }
}
