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

    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .fold(0, |acc, nums| acc + nums.0.abs_diff(nums.1))
}
