use std::str::from_utf8;

use crate::get_reg_operand_a;

fn get_combo_operand(operand: u8, a_reg: usize, b_reg: usize, c_reg: usize) -> usize {
    match operand {
        x if x <= b'3' => (x - b'0').into(),
        b'4' => a_reg,
        b'5' => b_reg,
        b'6' => c_reg,
        x => panic!("Invalid combo operand {:?}", x),
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
        .iter()
        .filter(|ch| ch != &&b',')
        .copied()
        .collect::<Vec<_>>();

    let mut prog_counter = 0;
    let mut output = vec![];

    while prog_counter < program.len() {
        let op_code = program[prog_counter];
        let operand = program[prog_counter + 1];

        match op_code {
            b'0' => {
                a_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            b'1' => b_reg ^= get_reg_operand_a(&operand),
            b'2' => b_reg = get_combo_operand(operand, a_reg, b_reg, c_reg) % 8,
            b'3' => {
                if a_reg != 0 {
                    prog_counter = get_reg_operand_a(&operand);
                    continue;
                }
            }
            b'4' => b_reg ^= c_reg,
            b'5' => output.push(get_combo_operand(operand, a_reg, b_reg, c_reg) % 8),
            b'6' => {
                b_reg = a_reg
                    / 2_usize.pow(
                        get_combo_operand(operand, a_reg, b_reg, c_reg)
                            .try_into()
                            .expect("Can u32 in 0"),
                    )
            }
            b'7' => {
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

    output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
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
