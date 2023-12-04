use std::collections::VecDeque;

fn main() {
    let input = aoc2023::get_input("04");

    let parsed = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| {
            let (win, have) = line.split_once('|').unwrap();

            let to_nums = |x: &str| -> Vec<usize> {
                x.split_whitespace().fold(Vec::new(), |mut nums, el| {
                    nums.push(el.parse().unwrap());
                    nums
                })
            };

            (to_nums(win), to_nums(have))
        });

    let p1 = parsed
        .clone()
        .map(|(win, have)| {
            have.into_iter()
                .filter(move |el| win.contains(&el))
                .fold(1, |acc, _| acc * 2)
                / 2
        })
        .sum::<usize>();

    let p2 = parsed
        .fold(
            (0, VecDeque::from([1usize])),
            |(total, mut acc), (win, have)| {
                let mult = acc.pop_front().unwrap_or(1);
                let wins = have.into_iter().filter(|x| win.contains(&x)).count();

                if acc.len() < wins {
                    acc.extend(vec![1; wins - acc.len()])
                }

                for m in acc.iter_mut().take(wins) {
                    *m += mult;
                }

                (total + mult, acc)
            },
        )
        .0;

    println!("{p1} {p2}");
}
