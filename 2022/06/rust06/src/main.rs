fn main() {
    let input = include_str!("../../input.txt");
    let input_vec = input.chars().collect::<Vec<_>>();

    let position = input_vec
        .windows(4)
        .position(|c| {
            !c.iter().enumerate().any(|(i, a)| c[i+1..].contains(a))
        })
        .unwrap();

    println!("Our Position {}", position + 4);
    
    let position = input_vec
        .windows(14)
        .position(|c| {
            !c.iter().enumerate().any(|(i, a)| c[i+1..].contains(a))
        })
        .unwrap();

    println!("Our Position {}", position + 14);
}
