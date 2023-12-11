use std::collections::HashSet;

pub fn process(input: &str, grow: usize) -> usize {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut galaxy_xs = HashSet::new();
    let mut galaxy_ys = HashSet::new();
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(
                move |(x, ch)| {
                    if *ch == '#' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<_>>();
    galaxies.iter().for_each(|(x, y)| {
        galaxy_xs.insert(*x);
        galaxy_ys.insert(*y);
    });

    let mut answer = 0;

    for (index, galaxy) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(index + 1) {
            let empty_xs = (galaxy.0.min(other.0)..other.0.max(galaxy.0))
                .filter(|u| !galaxy_xs.contains(u))
                .count();
            let empty_ys = (galaxy.1.min(other.1)..other.1.max(galaxy.1))
                .filter(|u| !galaxy_ys.contains(u))
                .count();

            let x_diff = galaxy.0.abs_diff(other.0);
            let y_diff = galaxy.1.abs_diff(other.1);
            answer +=
                (x_diff - empty_xs) + (empty_xs * grow) + (y_diff - empty_ys) + (empty_ys * grow);
        }
    }

    answer
}
