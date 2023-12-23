struct File {
    _name: String,
    size: i32,
}

impl File {
    fn new(name: &str, size: i32) -> File {
        File { _name: String::from(name), size }
    }
}

struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    size: i32,
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: String::from(name),
            files: Vec::new(),
            dirs: Vec::new(),
            size: 0,
        }
    }
}

fn main() {
    // File hosts.txt must exist in the current path
    let input = include_str!("../../input.txt");

    let mut top_dir = Dir::new("/");

    // Consumes the iterator, returns an (Optional) String
    let mut path: Vec<usize> = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let comm = line.strip_prefix("$ cd ").unwrap();
            match comm {
                "/" => path = Vec::new(),
                ".." => _ = path.pop(),
                x => {
                    let search_dir = if path.is_empty() { 
                        &top_dir 
                    } else { 
                        path.iter().fold(&top_dir, |prev, index| {
                            prev.dirs.get(*index).unwrap()
                        })
                    }; 
                    let new_position = search_dir.dirs.iter().position(|d| d.name == x).unwrap();
                    path.push(new_position);
                },
            };
        } else if line.starts_with("$ ls"){
            continue;
        } else if line.starts_with('d') {
           let current_dir = if path.is_empty() { 
                &mut top_dir
            } else { 
                path.iter().fold(&mut top_dir, |prev, index| {
                    prev.dirs.get_mut(*index).unwrap()
                })
            }; 

            current_dir.dirs.push(Dir::new(&line[4..]));
        } else if line.chars().next().unwrap().is_numeric() {
            let mut line_iter = line.split(' ');
            let size: i32 = line_iter.next().unwrap().parse().unwrap();
            let name = line_iter.next().unwrap();

            let current_dir = if path.is_empty() { 
                &mut top_dir
            } else { 
                path.iter().fold(&mut top_dir, |prev, index| {
                    prev.dirs.get_mut(*index).unwrap()
                })
            };

            current_dir.files.push(File::new(name, size));
        } else {
            println!("What?? {line}");
        }
    }

    let (top, unders) = calc_sizes(&mut top_dir);

    println!("We have a size {}", unders);

    let space_remaining = 70000000 - top;
    let space_needed = 30000000 - space_remaining;

    let smallest = find_smallest_above_needed(&top_dir, space_needed);
    println!("The smallest dir is {} with size {}", smallest.0, smallest.1);
}

fn calc_sizes(topdir: &mut Dir) -> (i32, i32) {
    let mut under100000s = 0;
    let mut size = topdir.files.iter().fold(0, |mut acc, f| { acc += f.size; acc });
    size += topdir.dirs.iter_mut().fold(0, |mut acc, d| {
        let (size, unders) = calc_sizes(d);
        acc += size;
        under100000s += unders;
        acc
    });
    topdir.size = size;
    if size < 100000 {
        under100000s += size;
    }

    (size, under100000s)
}

fn find_smallest_above_needed(topdir: &Dir, needed: i32) -> (String, i32) {
    let mut smallest = (topdir.name.clone(), topdir.size);
    topdir.dirs.iter().for_each(|d| {
        let new_small = find_smallest_above_needed(d, needed);
        if new_small.1 > needed && new_small.1 < smallest.1 {
            smallest = new_small;
        }
    });

    smallest
}