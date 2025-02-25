use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;
use rustc_hash::FxHashMap;

pub fn num_to_xy(ch: u8) -> (usize, usize) {
    match ch {
        b'0' => (1, 3),
        b'1' => (0, 2),
        b'2' => (1, 2),
        b'3' => (2, 2),
        b'4' => (0, 1),
        b'5' => (1, 1),
        b'6' => (2, 1),
        b'7' => (0, 0),
        b'8' => (1, 0),
        b'9' => (2, 0),
        b'A' => (2, 3),
        x => panic!("We shouldn't be doing {}", x),
    }
}

pub fn arrow_to_xy(ch: u8) -> (usize, usize) {
    match ch {
        b'^' => (1, 0),
        b'v' => (1, 1),
        b'>' => (2, 1),
        b'<' => (0, 1),
        b'A' => (2, 0),
        x => panic!("We shouldn't be doing {}", x),
    }
}

pub fn path_to_arrows(path: Vec<u8>) -> Vec<u8> {
    path.into_iter()
        .tuple_windows()
        .map(|step| match step {
            (b'7', b'8')
            | (b'8', b'9')
            | (b'4', b'5')
            | (b'5', b'6')
            | (b'1', b'2')
            | (b'2', b'3')
            | (b'0', b'A')
            | (b'<', b'v')
            | (b'v', b'>')
            | (b'^', b'A') => b'>',
            (b'8', b'7')
            | (b'9', b'8')
            | (b'5', b'4')
            | (b'6', b'5')
            | (b'2', b'1')
            | (b'3', b'2')
            | (b'A', b'0')
            | (b'v', b'<')
            | (b'>', b'v')
            | (b'A', b'^') => b'<',
            (b'1', b'4')
            | (b'4', b'7')
            | (b'0', b'2')
            | (b'2', b'5')
            | (b'5', b'8')
            | (b'A', b'3')
            | (b'3', b'6')
            | (b'6', b'9')
            | (b'v', b'^')
            | (b'>', b'A') => b'^',
            (b'4', b'1')
            | (b'7', b'4')
            | (b'2', b'0')
            | (b'5', b'2')
            | (b'8', b'5')
            | (b'3', b'A')
            | (b'6', b'3')
            | (b'9', b'6')
            | (b'^', b'v')
            | (b'A', b'>') => b'v',
            (x, y) if x == y => panic!("Same, twice"),
            x => panic!("Invalid step {:?}", x),
        })
        .collect::<Vec<_>>()
}

pub fn numpad_successors(n: &u8) -> std::vec::IntoIter<(u8, usize)> {
    match n {
        b'0' => vec![(b'A', 1), (b'2', 1)].into_iter(),
        b'1' => vec![(b'2', 1), (b'4', 1)].into_iter(),
        b'2' => vec![(b'0', 1), (b'1', 1), (b'3', 1), (b'5', 1)].into_iter(),
        b'3' => vec![(b'A', 1), (b'2', 1), (b'6', 1)].into_iter(),
        b'4' => vec![(b'1', 1), (b'5', 1), (b'7', 1)].into_iter(),
        b'5' => vec![(b'2', 1), (b'4', 1), (b'6', 1), (b'8', 1)].into_iter(),
        b'6' => vec![(b'3', 1), (b'5', 1), (b'9', 1)].into_iter(),
        b'7' => vec![(b'4', 1), (b'8', 1)].into_iter(),
        b'8' => vec![(b'5', 1), (b'7', 1), (b'9', 1)].into_iter(),
        b'9' => vec![(b'6', 1), (b'8', 1)].into_iter(),
        b'A' => vec![(b'0', 1), (b'3', 1)].into_iter(),
        x => panic!("We shouldn't be doing {}", x),
    }
}

pub fn arrow_successors(n: &u8) -> std::vec::IntoIter<(u8, usize)> {
    match n {
        b'^' => vec![(b'A', 1), (b'v', 1)].into_iter(),
        b'v' => vec![(b'^', 1), (b'<', 1), (b'>', 1)].into_iter(),
        b'>' => vec![(b'A', 1), (b'v', 1)].into_iter(),
        b'<' => vec![(b'v', 1)].into_iter(),
        b'A' => vec![(b'^', 1), (b'>', 1)].into_iter(),
        x => panic!("We shouldn't be doing {}", x),
    }
}

pub fn shorts_to_path_to_arrows(
    arrows: &[u8],
    times: usize,
    cache: &mut FxHashMap<(Vec<u8>, usize), Vec<u8>>,
) -> Vec<u8> {
    let as_vec = arrows.to_vec();
    if cache.contains_key(&(as_vec.clone(), times)) {
        return cache.get(&(as_vec, times)).unwrap().clone();
    }

    let mut paths = vec![];
    let mut elon_paths = vec![];

    let mut prev = b'A';

    for ch in arrows.iter().chain([&b'A']) {
        let exy = arrow_to_xy(*ch);

        let shortests = astar_bag_collect(
            &prev,
            arrow_successors,
            |n| {
                let nxy = arrow_to_xy(*n);

                nxy.0.abs_diff(exy.0) + nxy.1.abs_diff(exy.1)
            },
            |n| n == ch,
        )
        .expect("can path");

        if paths.is_empty() {
            shortests.0.into_iter().for_each(|path| {
                let mut n_path = path_to_arrows(path);
                n_path.push(b'A');

                paths.push(n_path);
            });
        } else {
            for extension in shortests.0 {
                let mut extension = path_to_arrows(extension);
                extension.push(b'A');

                for path in paths.iter() {
                    let mut n_path = path.clone();

                    for nch in extension.iter() {
                        n_path.push(*nch);
                    }

                    elon_paths.push(n_path);
                }
            }

            std::mem::swap(&mut paths, &mut elon_paths);
            elon_paths.clear();
        }

        prev = *ch;
    }

    let fin = if times == 1 {
        paths.remove(0)
    } else {
        // for path in paths.into_iter().take(5) {
        // for path in paths {
        //     let us_kiss = arrows_to_path_to_arrows(path, times - 1 /*, cache*/);

        //     if fin.is_empty() || fin.len() > us_kiss.len() {
        //         fin = us_kiss;
        //     }
        // }

        // let mut cache = FxHashMap::default();

        paths
            .into_iter()
            .map(|p| arrows_to_path_to_arrows(p, times - 1, cache))
            .fold(vec![], |acc, p| {
                if acc.is_empty() || acc.len() > p.len() {
                    p
                } else {
                    acc
                }
            })
        // .reduce(std::vec::Vec::new, |acc, p| {
        //     if acc.is_empty() || acc.len() > p.len() {
        //         p
        //     } else {
        //         acc
        //     }
        // })
    };

    cache.insert((arrows.to_vec(), times), fin.clone());
    fin
}

pub fn arrows_to_path_to_arrows(
    arrows: Vec<u8>,
    times: usize,
    cache: &mut FxHashMap<(Vec<u8>, usize), Vec<u8>>,
) -> Vec<u8> {
    let mut fin = vec![];
    for arrow_set in arrows.split(|ch| ch == &b'A') {
        fin.append(&mut shorts_to_path_to_arrows(arrow_set, times, cache))
    }

    // println!("Level times {}", times);

    fin
}
