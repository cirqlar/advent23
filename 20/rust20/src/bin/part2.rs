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

fn do_pulse(
    nodes: &mut HashMap<String, Node>,
    cache: &mut HashMap<(SentPulse, Node), Vec<SentPulse>>,
    cycles: &mut HashMap<String, (usize, usize)>,
    current_iter: usize,
) -> (usize, usize) {
    let mut pulses = VecDeque::from([SentPulse::new(
        Pulse::Low,
        String::from("broadcaster"),
        String::from("button"),
    )]);
    let mut rx_pulses = Vec::new();

    while !pulses.is_empty() {
        let sent_pulse = pulses.pop_front().unwrap();
        let SentPulse {
            pulse,
            sender,
            receiver: us,
        } = sent_pulse.clone();

        if us == *"ft" {
            println!("We got here but not necessarily there");
        }
        if us == *"ft" && pulse == Pulse::High {
            println!("Cycles for {} at {}", sender, current_iter);
            cycles
                .entry(sender.clone())
                .and_modify(|(start, end)| {
                    *start = *end;
                    *end = current_iter;
                })
                .or_insert((current_iter, current_iter));
        }

        if us == *"rx" {
            rx_pulses.push(pulse);
        }
        let Some(node) = nodes.get_mut(&us) else {
            continue;
        };

        if let Some(ap) = cache.get(&(sent_pulse.clone(), node.clone())) {
            pulses.append(&mut ap.clone().into());
            continue;
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
                Pulse::High => continue,
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

    let rx_high = rx_pulses.iter().filter(|p| *p == &Pulse::High).count();

    (rx_high, rx_pulses.len() - rx_high)
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

    let mut cache = HashMap::new();
    let mut cycles = HashMap::new();

    for i in 0..1_000_000_000 {
        let res = do_pulse(&mut nodes, &mut cache, &mut cycles, i);

        let Node::Conjunction(ft_node) = nodes.get("ft").unwrap() else {
            panic!("what??");
        };

        if cycles.keys().len() == ft_node.inputs.len()
            && cycles.values().all(|(start, end)| start != end)
        {
            let answer = lcm(&cycles
                .values()
                .map(|(start, end)| end - start)
                .collect::<Vec<_>>());

            println!("After run {} our answer is {}", i, answer);
            break;
        }

        if res.1 > 0 {
            println!("After run {} we have {} highs and {} lows", i, res.0, res.1);
            break;
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
