fn main() {
    let input = include_str!("../../input.txt");

    let sum = input
        .lines()
        .map(|line| {
            let len = line.len();
            (&line[..len/2], &line[len/2..])
        })
        .fold(0, |mut acc, split| {
            let c_index = split.0.find(|c| {
                split.1.find(|d| d == c).is_some()
            }).unwrap();

            let c = split.0[c_index..].chars().next().unwrap();

            acc += if c.is_uppercase() { (c as u32 - 'A' as u32) + 27 } else { (c as u32 - 'a' as u32) + 1 };
            acc
        });

    println!("Our sum {}", sum);

    let mut lines = input.lines();
    let mut sum = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let c_index = line1.find(|c| {
            line2.find(|d| d == c).is_some() && line3.find(|d| d == c).is_some()
        }).unwrap();

        let c = line1[c_index..].chars().next().unwrap();
        sum += if c.is_uppercase() { (c as u32 - 'A' as u32) + 27 } else { (c as u32 - 'a' as u32) + 1 };
    };

    println!("Our sum {}", sum);
}
