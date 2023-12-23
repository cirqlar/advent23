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

#[derive(Debug)]
struct PartRange {
    x_start: usize,
    x_end: usize,
    m_start: usize,
    m_end: usize,
    a_start: usize,
    a_end: usize,
    s_start: usize,
    s_end: usize,
}

impl PartRange {
    fn split_x(self, position: usize) -> (PartRange, PartRange) {
        (
            PartRange {
                x_end: position,
                ..self
            },
            PartRange {
                x_start: position + 1,
                ..self
            },
        )
    }

    fn split_m(self, position: usize) -> (PartRange, PartRange) {
        (
            PartRange {
                m_end: position,
                ..self
            },
            PartRange {
                m_start: position + 1,
                ..self
            },
        )
    }

    fn split_a(self, position: usize) -> (PartRange, PartRange) {
        (
            PartRange {
                a_end: position,
                ..self
            },
            PartRange {
                a_start: position + 1,
                ..self
            },
        )
    }

    fn split_s(self, position: usize) -> (PartRange, PartRange) {
        (
            PartRange {
                s_end: position,
                ..self
            },
            PartRange {
                s_start: position + 1,
                ..self
            },
        )
    }
}

fn check_parts(range: PartRange, workflows: &HashMap<String, Ops>, entry: &str) -> usize {
    let ops = workflows.get(entry).unwrap();

    let mut rem_parts = range;

    let mut next = Vec::new();

    for op in ops.ops.iter() {
        let split_pos = op.cmp - if op.op == '<' { 1 } else { 0 };

        let (less, great) = match op.cats {
            'x' => rem_parts.split_x(split_pos),
            'm' => rem_parts.split_m(split_pos),
            'a' => rem_parts.split_a(split_pos),
            's' => rem_parts.split_s(split_pos),
            _ => unreachable!(),
        };

        if op.op == '<' {
            rem_parts = great;
            next.push((op.dst.clone(), less));
        } else {
            rem_parts = less;
            next.push((op.dst.clone(), great));
        }
    }

    next.push((ops.end.clone(), rem_parts));

    let mut total = 0;

    for (s, p) in next {
        if s == "A" {
            let x_count = p.x_end - p.x_start + 1;
            let m_count = p.m_end - p.m_start + 1;
            let a_count = p.a_end - p.a_start + 1;
            let s_count = p.s_end - p.s_start + 1;
            let total_count = x_count * m_count * a_count * s_count;

            total += total_count;
            continue;
        } else if s == "R" {
            continue;
        }

        total += check_parts(p, workflows, &s);
    }

    total
}

fn process(input: &str) -> usize {
    let delim = if cfg!(windows) { "\r\n\r\n" } else { "\n\n" };
    let mut split = input.split(delim);
    let workflows = split.next().unwrap();

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

    let range = PartRange {
        x_start: 1,
        x_end: 4000,
        m_start: 1,
        m_end: 4000,
        a_start: 1,
        a_end: 4000,
        s_start: 1,
        s_end: 4000,
    };
    check_parts(range, &workflows, "in")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d19_part_2_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 167409079868000);
    }
}
