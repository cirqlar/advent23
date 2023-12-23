use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Broadcaster {
    outputs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OnOff {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    state: OnOff,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

struct SentPulse {
    pulse: Pulse,
    sender: String,
    receiver: String,
}

impl SentPulse {
    fn new(p: Pulse, r: String, s: String) -> SentPulse {
        Self {
            pulse: p,
            sender: s,
            receiver: r,
        }
    }
}

fn do_pulse(nodes: &mut HashMap<String, Node>) -> (usize, usize) {
    let mut low_count = 0;
    let mut high_count = 0;

    let mut pulses = VecDeque::from([SentPulse::new(
        Pulse::Low,
        String::from("broadcaster"),
        String::from("button"),
    )]);

    while !pulses.is_empty() {
        let SentPulse {
            pulse,
            sender,
            receiver: us,
        } = pulses.pop_front().unwrap();
        if pulse == Pulse::High {
            high_count += 1;
        } else {
            low_count += 1;
        }
        let Some(node) = nodes.get_mut(&us) else {
            continue;
        };

        match node {
            Node::Broadcaster(b) => {
                for out in b.outputs.iter().cloned() {
                    pulses.push_back(SentPulse::new(pulse, out, us.clone()));
                }
            }
            Node::FlipFlop(f) => match pulse {
                Pulse::Low => {
                    f.state = match f.state {
                        OnOff::On => OnOff::Off,
                        OnOff::Off => OnOff::On,
                    };
                    let pulse_to_send = match f.state {
                        OnOff::On => Pulse::High,
                        OnOff::Off => Pulse::Low,
                    };

                    for out in f.outputs.iter().cloned() {
                        pulses.push_back(SentPulse::new(pulse_to_send, out, us.clone()));
                    }
                }
                Pulse::High => continue,
            },
            Node::Conjunction(c) => {
                c.inputs.entry(sender).and_modify(|p| *p = pulse);
                let mut pulse_to_send = Pulse::High;

                if c.inputs.values().all(|v| v == &Pulse::High) {
                    pulse_to_send = Pulse::Low;
                }

                for out in c.outputs.iter().cloned() {
                    pulses.push_back(SentPulse::new(pulse_to_send, out, us.clone()));
                }
            }
        }
    }

    (high_count, low_count)
}

fn process(input: &str) -> usize {
    let mut nodes = input
        .lines()
        .map(|line| {
            let (label, outputs) = line.split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").map(String::from).collect();

            match label.chars().next().unwrap() {
                '%' => (
                    String::from(&label[1..]),
                    Node::FlipFlop(FlipFlop {
                        state: OnOff::Off,
                        outputs,
                    }),
                ),
                '&' => (
                    String::from(&label[1..]),
                    Node::Conjunction(Conjunction {
                        inputs: HashMap::new(),
                        outputs,
                    }),
                ),
                'b' => (
                    String::from(label),
                    Node::Broadcaster(Broadcaster { outputs }),
                ),
                _ => unreachable!("Is of known node type"),
            }
        })
        .collect::<HashMap<_, _>>();

    let keys = nodes.keys().cloned().collect::<Vec<_>>();

    for key in keys {
        let outputs = match nodes.get(&key).unwrap() {
            Node::Broadcaster(b) => b.outputs.clone(),
            Node::FlipFlop(f) => f.outputs.clone(),
            Node::Conjunction(c) => c.outputs.clone(),
        };

        for out in outputs {
            if let Some(Node::Conjunction(c)) = nodes.get_mut(&out) {
                c.inputs.insert(key.clone(), Pulse::Low);
            }
        }
    }
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let res = do_pulse(&mut nodes);
        high += res.0;
        low += res.1;
    }

    high * low
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d20_part_1_is_correct_01() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(process(input), 32000000);
    }

    #[test]
    fn d20_part_1_is_correct_02() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(process(input), 11687500);
    }
}
