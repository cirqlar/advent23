use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug)]
struct Op {
    cats: char,
    op: char,
    cmp: usize,
    dst: String,
}

#[derive(Debug)]
struct Ops {
    ops: Vec<Op>,
    end: String,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn process(input: &str) -> usize {
    let delim = if cfg!(windows) { "\r\n\r\n" } else { "\n\n" };
    let mut split = input.split(delim);
    let workflows = split.next().unwrap();
    let parts = split.next().unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (label, rest) = line.split_once('{').unwrap();
            let rest = rest[..(rest.len() - 1)].split(',').collect::<Vec<_>>();

            let ops = Ops {
                ops: rest
                    .iter()
                    .take(rest.len() - 1)
                    .map(|op| {
                        let mut chs = op.chars();
                        let mut others = op[2..].split(':');

                        Op {
                            cats: chs.next().unwrap(),
                            op: chs.next().unwrap(),
                            cmp: others.next().unwrap().parse().unwrap(),
                            dst: others.next().unwrap().into(),
                        }
                    })
                    .collect(),
                end: (*rest.last().unwrap()).into(),
            };

            (String::from(label), ops)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line| {
            let mut iter = line[1..(line.len() - 1)].split(',');

            Part {
                x: (iter.next().unwrap())[2..].parse().unwrap(),
                m: (iter.next().unwrap())[2..].parse().unwrap(),
                a: (iter.next().unwrap())[2..].parse().unwrap(),
                s: (iter.next().unwrap())[2..].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    parts
        .iter()
        .filter_map(|p| {
            let mut next = "in";

            while next != "A" && next != "R" {
                let ops = workflows.get(next).unwrap();

                let mut found = false;
                for op in ops.ops.iter() {
                    let check_val = match op.cats {
                        'x' => p.x,
                        'm' => p.m,
                        'a' => p.a,
                        's' => p.s,
                        _ => unreachable!(),
                    };
                    let check = match op.op {
                        '<' => check_val < op.cmp,
                        '>' => check_val > op.cmp,
                        _ => unreachable!(),
                    };

                    if check {
                        next = &op.dst;
                        found = true;
                        break;
                    }
                }

                if !found {
                    next = &ops.end;
                }
            }

            if next == "R" {
                None
            } else {
                Some(p.x + p.m + p.a + p.s)
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d19_part_1_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 19114);
    }
}
