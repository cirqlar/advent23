

fn main() {
    let input = include_str!("../../input.txt");

    let mut max = [0; 3];

    input.split("\n\n").for_each(|lines| {
        let mut sum: i32 = lines
            .lines()
            .filter_map(|s| s.parse::<i32>().ok())
            .sum();

        for m in max.iter_mut() {
            if m < &mut sum {          
                std::mem::swap(m, &mut sum);
            }
        }
    });

    println!("The final {:?} and sum {}", max, max.iter().sum::<i32>());
}
