use std::collections::HashMap;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn main() {
    let input = include_str!("../../input/08.txt");

    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let lvec = line.split(" = ").collect::<Vec<&str>>();

            let vals = lvec[1].split(", ").collect::<Vec<&str>>();
            let left = vals[0].chars().skip(1).collect::<String>();
            let right = vals[1].chars().take(3).collect::<String>();

            (lvec[0], (left, right))
        })
        .fold(
            HashMap::<&str, (String, String)>::new(),
            |mut map, (key, (left, right))| {
                map.insert(key, (left, right));
                map
            },
        );

    let p1 = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .cycle()
        .try_fold(("AAA", 1usize), |(key, count), direction| {
            let dest = map.get(key).unwrap();

            let nkey = match direction {
                'L' => &dest.0,
                'R' => &dest.1,
                _ => panic!("Invalid direction!"),
            };

            if nkey == "ZZZ" {
                return Err(count);
            }

            Ok((nkey, count + 1))
        })
        .unwrap_err();

    let p2 = map
        .keys()
        .filter(|key| key.chars().next_back().unwrap() == 'A')
        .copied()
        .map(|start| {
            input
                .lines()
                .nth(0)
                .unwrap()
                .chars()
                .cycle()
                .try_fold((start, 1usize), |(key, count), direction| {
                    let dest = map.get(key).unwrap();
                    let nkey = match direction {
                        'L' => &dest.0,
                        'R' => &dest.1,
                        _ => panic!("Invalid direction!"),
                    };
                    if nkey.chars().next_back().unwrap() == 'Z' {
                        return Err(count);
                    }
                    Ok((nkey, count + 1))
                })
                .unwrap_err()
        })
        .fold(1usize, lcm);

    println!("{p1} {p2}");
}
