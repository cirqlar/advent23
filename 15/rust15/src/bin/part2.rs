use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn hash(input: &str) -> u8 {
    let mut current_value: u32 = 0;
    for c in input.chars() {
        current_value += (c as u8) as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value as u8
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn process(input: &str) -> usize {
    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    input.split(',').for_each(|seq| {
        let seq = seq.trim();

        if seq.contains('-') {
            let label = seq.split('-').next().expect("has label");
            let box_num = hash(label);

            boxes.entry(box_num).and_modify(|b| {
                if let Some(x) = b.iter().position(|l| l.label == label) {
                    b.remove(x);
                }
            });
        } else {
            let mut iter = seq.split('=');
            let label = iter.next().expect("has label");
            let focal = iter
                .next()
                .expect("has focal")
                .parse::<u8>()
                .expect("parsable");

            let box_num = hash(label);

            let lens = Lens {
                label: label.into(),
                focal_length: focal,
            };

            boxes
                .entry(box_num)
                .and_modify(|b| {
                    if let Some(x) = b.iter().position(|l| l.label == label) {
                        b[x].focal_length = focal;
                    } else {
                        b.push(lens.clone());
                    }
                })
                .or_insert(vec![lens]);
        }
    });

    let mut combo: usize = 0;

    for (key, value) in boxes.into_iter() {
        for (index, lens) in value.into_iter().enumerate() {
            combo += ((key as usize) + 1) * (index + 1) * (lens.focal_length as usize);
        }
    }

    combo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d15_part_2_is_correct() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(process(input), 145);
    }
}
