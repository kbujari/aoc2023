use std::collections::HashMap;

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

const fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn to_map(data: &str) -> HashMap<&str, (&str, &str)> {
    data.lines()
        .skip(2)
        .map(|line| {
            let lvec = line.split(" = ").collect::<Vec<&str>>();

            let vals = lvec[1].split(", ").collect::<Vec<&str>>();
            let left = vals[0].strip_prefix('(').unwrap();
            let right = vals[1].strip_suffix(')').unwrap();

            (lvec[0], (left, right))
        })
        .fold(HashMap::new(), |mut map, (key, (left, right))| {
            map.insert(key, (left, right));
            map
        })
}

fn main() {
    let input = include_str!("../../input/08.txt");
    let map = to_map(input);
    let directions = input.split_once('\n').unwrap().0.chars().cycle();

    let p1 = directions
        .clone()
        .try_fold(("AAA", 1usize), |(key, count), direction| {
            let (left, right) = map.get(key).unwrap();

            let nkey = match direction {
                'L' => left,
                'R' => right,
                _ => unreachable!(),
            };

            if *nkey == "ZZZ" {
                return Err(count);
            }

            Ok((nkey, count + 1))
        })
        .unwrap_err();

    let p2 = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|node| {
            directions
                .clone()
                .try_fold((node, 1usize), |(key, count), direction| {
                    let (left, right) = map.get(key).unwrap();
                    let nkey = match direction {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    };

                    if nkey.ends_with('Z') {
                        return Err(count);
                    }

                    Ok((nkey, count + 1))
                })
                .unwrap_err()
        })
        .reduce(lcm)
        .unwrap();

    println!("{p1} {p2}");
}
