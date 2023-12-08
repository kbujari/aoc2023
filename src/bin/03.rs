use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Unit {
    Dot,
    Number(char),
    Symbol,
    Gear,
}

fn into_map(data: &str) -> HashMap<(isize, isize), Unit> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, c)| (col, row, c)))
        .fold(HashMap::new(), |mut map, (x, y, unit)| {
            let u = if unit.is_ascii_digit() {
                Unit::Number(unit)
            } else if unit == '*' {
                Unit::Gear
            } else if unit != '.' && unit.is_ascii_punctuation() {
                Unit::Symbol
            } else {
                Unit::Dot
            };

            map.insert((x as isize, y as isize), u);
            map
        })
}

fn main() {
    let input = aoc2023::get_input("03");
    let map = into_map(&input);
    let mut gear_map: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
    let len = input.lines().count();

    let mut p1 = 0;

    for y in 0..len {
        let y = y as isize;
        let mut x = 0isize;
        while x < len as isize {
            let unit = map.get(&(x, y)).unwrap();

            if let Unit::Number(ch) = unit {
                let mut temp: String = String::with_capacity(3);
                let mut lookahead = 1;
                temp.push(*ch);

                while let Some(Unit::Number(nested_ch)) = map.get(&(x + lookahead, y)) {
                    temp.push(*nested_ch);
                    lookahead += 1;
                }

                let mut neighbours = vec![(x - 1, y), (x + lookahead, y)];
                for idx in 0..(temp.len() + 2) {
                    neighbours.push((x - 1 + idx as isize, y - 1));
                    neighbours.push((x - 1 + idx as isize, y + 1));
                }

                let symbol_adjacent = neighbours.into_iter().any(|unit| match map.get(&unit) {
                    Some(Unit::Symbol) => true,
                    Some(Unit::Gear) => {
                        gear_map
                            .entry(unit)
                            .or_default()
                            .push(temp.parse().unwrap());

                        true
                    }
                    _ => false,
                });

                if symbol_adjacent {
                    p1 += temp.parse::<usize>().unwrap();
                }

                x += lookahead;
            }

            x += 1;
        }
    }

    let p2 = gear_map
        .values()
        .filter_map(|val| {
            if val.len() == 2 {
                Some(val.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{p1} {p2}");
}
