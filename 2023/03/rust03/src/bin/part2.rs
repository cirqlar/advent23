use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer: {}", answer);
}

fn from_1d_to_2d(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}
fn from_2d_to_1d(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
fn is_new_line(index: usize, width: usize) -> bool {
    index % width == 0
}

struct Box {
    begin: (i32, i32),
    width: usize,
    index: usize,
    max: usize,
    bounds: (usize, usize),
}

impl Box {
    fn new(begin: (i32, i32), width: usize, bounds: (usize, usize)) -> Box {
        Box {
            begin,
            width,
            index: 0,
            max: width * 2 + 2 - 1,
            bounds,
        }
    }

    fn update(&mut self, begin: (i32, i32), width: usize) {
        self.begin = begin;
        self.width = width;
        self.max = width * 2 + 2 - 1;
        self.index = 0;
    }
}

impl Iterator for &mut Box {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.max {
            return None;
        }
        let index: i32 = self.index.try_into().ok()?;
        let width: i32 = self.width.try_into().ok()?;

        let mut the_x = self.begin.0;
        let mut the_y = self.begin.1;

        if index < width {
            the_x += index;
        } else if index < width + 2 {
            the_x = if index == width {
                the_x
            } else {
                the_x + width - 1
            };
            the_y += 1;
        } else {
            the_x += index - width - 2;
            the_y += 2;
        }

        self.index += 1;

        if the_x >= 0
            && the_x < self.bounds.0.try_into().ok()?
            && the_y >= 0
            && the_y < self.bounds.1.try_into().ok()?
        {
            Some(from_2d_to_1d(
                the_x.try_into().ok()?,
                the_y.try_into().ok()?,
                self.bounds.0,
            ))
        } else {
            self.next()
        }
    }
}

fn process(input: &str) -> i32 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let lines: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();

    let mut our_box = Box::new((0, 0), 0, (width, height));

    lines
        .iter()
        .enumerate()
        .fold(("".to_string(), 0, HashMap::new()), |mut acc, cur| {
            if (is_new_line(cur.0, width) || !cur.1.is_ascii_digit()) && !acc.0.is_empty() {
                let begin = from_1d_to_2d(acc.1, width);
                let begin: (i32, i32) = (begin.0.try_into().unwrap(), begin.1.try_into().unwrap());
                let begin = (begin.0 - 1, begin.1 - 1);

                our_box.update(begin, cur.0 - acc.1 + 2);

                for index in &mut our_box {
                    let ch = lines[index];
                    if ch == '*' {
                        let as_num: i32 =
                            acc.0.parse().expect("string should be parsable as number");

                        acc.2
                            .entry(index)
                            .and_modify(|v: &mut Vec<i32>| v.push(as_num))
                            .or_insert(vec![as_num]);

                        break;
                    }
                }

                acc.0.clear();
            }
            if cur.1.is_ascii_digit() {
                if acc.0.is_empty() {
                    acc.1 = cur.0;
                }
                acc.0.push(*cur.1);
            }

            acc
        })
        .2
        .into_iter()
        .filter_map(|v| {
            if v.1.len() == 2 {
                Some(v.1[0] * v.1[1])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d03_part_2_is_correct() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(process(input), 467835);
    }
}
