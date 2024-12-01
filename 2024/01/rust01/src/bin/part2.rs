use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let result = process(input);

    println!("Answer {result}");
}

fn process(input: &str) -> u32 {
    let mut first = Vec::new();
    let mut second = Vec::new();

    input
        .lines()
        .map(|line| {
            let mut temp = line.trim().split("   ");
            [
                temp.next().expect("Can get next here"),
                temp.next().expect("Can get next here"),
            ]
        })
        .for_each(|each| {
            first.push(each[0].parse::<u32>().expect("First num is parsable"));
            second.push(each[1].parse::<u32>().expect("Second num is parsable"));
        });

    let mut mappss: HashMap<u32, u32> = HashMap::new();

    second.into_iter().for_each(|num| {
        mappss.entry(num).and_modify(|e| *e += 1).or_insert(1);
    });

    first
        .into_iter()
        .fold(0, |acc, num| acc + (num * mappss.get(&num).unwrap_or(&0)))
}
