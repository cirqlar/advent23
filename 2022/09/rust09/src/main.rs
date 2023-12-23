use std::collections::HashSet;

fn sign(num: i32) -> i32 {
    match num {
        1.. => 1,
        0 => 0,
        _ => -1,  
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    
    let mut visited_1 = HashSet::new();
    let mut visited_2 = HashSet::new();
    
    let mut rope = [(0, 0); 10];
    
    for line in input.lines() {
        let direction = &line[0..1];
        let times =  line[2..].parse::<usize>().unwrap();
        
        for _i in 0..times {
            match direction {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("What's popping"),
            }

            for i in 1..10 {
                let x_distance = rope[i-1].0 - rope[i].0;
                let y_distance = rope[i-1].1 - rope[i].1;

                if i32::abs(x_distance) > 1 || i32::abs(y_distance) > 1 {
                    rope[i].0 += sign(x_distance);
                    rope[i].1 += sign(y_distance);
                }

                if i == 1 {
                    visited_1.insert(rope[i]);
                }
                if i == 9 {
                    visited_2.insert(rope[i]);
                }
            }
        }
    }

    println!("1 visits{} and 9 visits {}", visited_1.len(), visited_2.len());
}

