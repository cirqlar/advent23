use crate::operations::Operation;
use itertools::Itertools;
use std::str::from_utf8;

pub fn process(input: &[u8]) -> String {
    let last_colon = input.len()
        - 1
        - input
            .iter()
            .rev()
            .position(|ch| ch == &b':')
            .expect("Exists");
    let split_position = last_colon
        + input[last_colon..]
            .iter()
            .position(|ch| ch == &b'\n')
            .expect("Exists");

    let mut largest_z = [0; 3];

    let commands = input[split_position..]
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut a = [0; 3];
            let mut b = [0; 3];
            let mut c = [0; 3];

            let ret = match &line[4..7] {
                b"AND" => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[8..11]);
                    c.copy_from_slice(&line[15..18]);

                    Operation::And(a, b, c)
                }
                b"XOR" => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[8..11]);
                    c.copy_from_slice(&line[15..18]);

                    Operation::Xor(a, b, c)
                }
                b"OR " => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[7..10]);
                    c.copy_from_slice(&line[14..17]);

                    Operation::Orr(a, b, c)
                }
                x => unreachable!("We don't have any other commands {}", from_utf8(x).unwrap()),
            };

            if c[0] == b'z' && c > largest_z {
                largest_z = c;
            }

            ret
        })
        .collect::<Vec<_>>();

    let bad_commands = commands
        .iter()
        .filter(|command| match command {
            Operation::And(first, _, out) => {
                // An And should never be connected to a final output
                if out[0] == b'z' {
                    true
                } else {
                    let as_input_commands = find_has_input(out, &commands);
                    if as_input_commands.len() > 1 {
                        // Every And exlcuding x00 AND y00 should be connected to 1 command
                        first != b"x00" && first != b"y00"
                    } else {
                        // Every And connected to one output should be connected to an Orr
                        !as_input_commands[0].is_orr()
                    }
                }
            }
            Operation::Orr(_, _, out) => {
                // An Orr should never be connected to a final output excluding the last final output
                if out[0] == b'z' {
                    out != &largest_z
                } else {
                    let as_input_commands = find_has_input(out, &commands);

                    if as_input_commands.len() != 2 {
                        // Every Orr must be connected to 2 commands
                        // Except the one connected to the last final output but we shouldn't be here if that's the case for this Orr
                        true
                    } else {
                        // Every Orr must be connected to one Xor and one And
                        !((as_input_commands[0].is_xor() && as_input_commands[1].is_and())
                            || (as_input_commands[1].is_xor() && as_input_commands[0].is_and()))
                    }
                }
            }
            Operation::Xor(first, _, out) => {
                // An Xor can be connected to a final output
                if out[0] == b'z' {
                    false
                } else {
                    let as_input_commands = find_has_input(out, &commands);

                    if as_input_commands.len() != 2 {
                        // An Xor that isn't connected to a final output must be connected to two commands
                        true
                    } else {
                        // An Xor connected to two commands must have x and y inputs and be connected to an And and an Xor
                        !(first[0] == b'x' || first[0] == b'y')
                            || !((as_input_commands[0].is_xor() && as_input_commands[1].is_and())
                                || (as_input_commands[1].is_xor() && as_input_commands[0].is_and()))
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    bad_commands
        .into_iter()
        .map(|op| op.get_c().iter().map(|c| *c as char).join(""))
        .sorted()
        .join(",")
}

fn find_has_input<'a>(input: &[u8], commands: &'a [Operation]) -> Vec<&'a Operation> {
    commands
        .iter()
        .filter(|command| command.get_a() == input || command.get_b() == input)
        .collect()
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_1() {
//         let input = include_bytes!("../../input/part1_example.txt");

//         let answer = super::process(input);

//         assert_eq!(answer, 2024);
//     }
// }
