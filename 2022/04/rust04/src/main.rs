fn main() {
    let input = include_str!("../../input.txt");

    let contains = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.split('-'))
                .flat_map(|s| s.filter_map(|s| s.parse::<i32>().ok()))
                .collect::<Vec<_>>()
        })
        .fold(0, |mut acc, ranges| {
            if (ranges[0] <= ranges[2] && ranges[1] >= ranges[3])
                || (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
            {
                acc += 1;
            }

            acc
        });

    println!("We have {contains}");

    let contains = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.split('-'))
                .flat_map(|s| s.filter_map(|s| s.parse::<i32>().ok()))
                .collect::<Vec<_>>()
        })
        .fold(0, |mut acc, ranges| {
            if (ranges[1] <= ranges[3] && ranges[1] >= ranges[2])
                || (ranges[3] <= ranges[1] && ranges[3] >= ranges[0])
            {
                acc += 1;
            }

            acc
        });

    println!("We have {contains}");
}
