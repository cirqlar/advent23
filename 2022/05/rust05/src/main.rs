

fn main() {
    let input = include_str!("../../input.txt");
    let (stacklist, commands) = input.split_once("\n\n").unwrap();
    let num_of_stacks = stacklist.chars().rev().nth(1).unwrap().to_string().parse::<usize>().unwrap();

    let mut stacks = vec![Vec::new(); num_of_stacks];

    for line in stacklist.lines().rev().skip(1) {
        let chars = line.chars().collect::<Vec<_>>();
        for (index, c) in chars.chunks(4).enumerate() {
            if c[1].is_alphabetic() { stacks[index].push(c[1]) };
        }
    }

    for comm in commands.lines() {
        let mut comm_it = comm.split(' ');
        comm_it.next();
        let move_amt: usize = comm_it.next().unwrap().parse().unwrap();
        comm_it.next();
        let from_ind: usize = comm_it.next().unwrap().parse().unwrap();
        comm_it.next();
        let to_ind: usize = comm_it.next().unwrap().parse().unwrap();

        (0..move_amt).for_each(|_i| {
            let el = stacks[from_ind-1].pop().unwrap();
            stacks[to_ind-1].push(el);
        });
    }

    println!("We get {}", stacks.iter().map(|s| s.last().unwrap()).collect::<String>() );
    
    let mut stacks = vec![Vec::new(); num_of_stacks];

    for line in stacklist.lines().rev().skip(1) {
        let chars = line.chars().collect::<Vec<_>>();
        for (index, c) in chars.chunks(4).enumerate() {
            if c[1].is_alphabetic() { stacks[index].push(c[1]) };
        }
    }

    for comm in commands.lines() {
        let mut comm_it = comm.split(' ');
        comm_it.next();
        let move_amt: usize = comm_it.next().unwrap().parse().unwrap();
        comm_it.next();
        let from_ind: usize = comm_it.next().unwrap().parse().unwrap();
        comm_it.next();
        let to_ind: usize = comm_it.next().unwrap().parse().unwrap();

        let from_len = stacks[from_ind-1].len();
        let mut tail = stacks[from_ind-1].split_off(from_len - move_amt);
        stacks[to_ind-1].append(&mut tail);
    }

    println!("We get {}", stacks.iter().map(|s| s.last().unwrap()).collect::<String>() );
}
