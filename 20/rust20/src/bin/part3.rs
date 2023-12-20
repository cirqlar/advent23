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

    let mut cache: HashMap<(SentPulse, Node), Vec<SentPulse>> = HashMap::new();
    // let mut cycles = HashMap::new();
    // let Node::Conjunction(ft_node) = nodes.get("ft").unwrap() else {
    //     panic!("what??");
    // };
    // let ft_inputs = ft_node.inputs.keys().cloned().collect::<Vec<_>>();
    let checks: [String; 4] = ["sl".into(), "jz".into(), "pq".into(), "rr".into()];
    let mut check_cyclics = HashMap::new();
    let mut final_cyclics = HashMap::new();

    'overall: for i in 1..1_000_000_000_usize {
        if i % 1_000_000 == 0 {
            println!("Iter {}", i);
        }
        let mut pulses = VecDeque::from([SentPulse::new(
            Pulse::Low,
            String::from("broadcaster"),
            String::from("button"),
        )]);

        'pulses: while !pulses.is_empty() {
            let sent_pulse = pulses.pop_front().unwrap();
            let SentPulse {
                pulse,
                sender,
                receiver: us,
            } = sent_pulse.clone();

            let Some(node) = nodes.get_mut(&us) else {
                continue 'pulses;
            };

            if us == *"qh" && pulse == Pulse::Low {
                println!("I {}", i);
                break 'overall;
            }

            if checks.contains(&us) && !final_cyclics.contains_key(&us) && pulse == Pulse::High {
                if !check_cyclics.contains_key(&us) {
                    check_cyclics.insert(us.clone(), HashMap::new());
                }
                let ref_m = check_cyclics.get_mut(&us).unwrap();
                if !ref_m.contains_key(&sender) {
                    println!("Cycles for {} from {} at {}", us, sender, i);
                    ref_m.insert(sender.clone(), i);
                    let Node::Conjunction(we) = node else {
                        panic!("how");
                    };

                    if ref_m.len() == we.inputs.len() {
                        println!("Final Cycles for {} from {} at {}", us, sender, i);
                        let answer = lcm(&ref_m.values().cloned().collect::<Vec<_>>());
                        final_cyclics.insert(us.clone(), answer);

                        if final_cyclics.len() == checks.len() {
                            let answer = lcm(&final_cyclics.values().cloned().collect::<Vec<_>>());
                            println!("After run {} our answer is {}", i, answer);
                            break 'overall;
                        }
                    }
                }
            }

            // if us == *"ft" && pulse == Pulse::High {
            //     println!("Cycles for {} at {}", sender, i);
            //     cycles
            //         .entry(sender.clone())
            //         // .and_modify(|(start, end)| {
            //         //     *start = *end;
            //         //     *end = i;
            //         // })
            //         .or_insert(i);

            //     if cycles.len() == ft_inputs.len()
            //     // && cycles.values().all(|(start, end)| start != end)
            //     {
            //         let answer = lcm(&cycles
            //             .values()
            //             .cloned()
            //             // .map(|(start, end)| end - start)
            //             .collect::<Vec<_>>());

            //         println!("After run {} our answer is {}", i, answer);
            //         break 'overall;
            //     }
            // }

            if let Some(ap) = cache.get(&(sent_pulse.clone(), node.clone())) {
                pulses.append(&mut ap.clone().into());
                continue 'pulses;
            }

            let old_node = node.clone();
            let mut append_pulses = Vec::new();

            match node {
                Node::Broadcaster(b) => {
                    for out in b.outputs.iter().cloned() {
                        append_pulses.push(SentPulse::new(pulse, out, us.clone()));
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
                            append_pulses.push(SentPulse::new(pulse_to_send, out, us.clone()));
                        }
                    }
                    Pulse::High => continue 'pulses,
                },
                Node::Conjunction(c) => {
                    c.inputs.entry(sender).and_modify(|p| *p = pulse);
                    let mut pulse_to_send = Pulse::High;

                    if c.inputs.values().all(|v| v == &Pulse::High) {
                        pulse_to_send = Pulse::Low;
                    }

                    for out in c.outputs.iter().cloned() {
                        append_pulses.push(SentPulse::new(pulse_to_send, out, us.clone()));
                    }
                }
            }

            cache.insert((sent_pulse, old_node), append_pulses.clone());
            pulses.append(&mut append_pulses.into());
        }
    }

    0
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
