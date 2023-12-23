use std::collections::{BTreeMap, HashMap, VecDeque};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Broadcaster {
    outputs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OnOff {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlipFlop {
    state: OnOff,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Conjunction {
    inputs: BTreeMap<String, Pulse>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

fn gcd(a: usize, b: usize) -> usize {
    // println!("comparying {} and {}", a, b);
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

// fn do_pulse(
//     nodes: &mut HashMap<String, Node>,
//     cache: &mut ,
//     cycles: &mut HashMap<String, (usize, usize)>,
//     current_iter: usize,

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
                        inputs: BTreeMap::new(),
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

    let Node::Conjunction(prev_node) = nodes
        .iter()
        .find(|(_, v)| {
            if let Node::Conjunction(v) = v {
                v.outputs.contains(&"rx".into())
            } else {
                false
            }
        })
        .unwrap()
        .1
    else {
        panic!("whys")
    };
    let prev_node_inputs = prev_node.inputs.keys().cloned().collect::<Vec<_>>();
    let mut input_num_cycle = HashMap::new();

    'overall: for i in 1.. {
        let mut pulses = VecDeque::from([SentPulse::new(
            Pulse::Low,
            String::from("broadcaster"),
            String::from("button"),
        )]);

        while !pulses.is_empty() {
            let sent_pulse = pulses.pop_front().unwrap();
            let SentPulse {
                pulse,
                sender,
                receiver: us,
            } = sent_pulse.clone();

            let Some(node) = nodes.get_mut(&us) else {
                continue;
            };

            if prev_node_inputs.contains(&us)
                && pulse == Pulse::Low
                && !input_num_cycle.contains_key(&us)
            {
                input_num_cycle.insert(us.clone(), i);

                if input_num_cycle.len() == prev_node_inputs.len() {
                    break 'overall;
                }
            }

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
    }

    lcm(&input_num_cycle.values().cloned().collect::<Vec<_>>())
}
