fn main() {
    // let input = include_str!("../../input/09.txt");
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let p1 =
        input
            .lines()
            .map(|line| line.split_whitespace())
            .map(|line| line.map(|num| num.parse::<isize>().unwrap()))
            .map(|nums| nums.collect::<Vec<isize>>())
            .map(|init| vec![init])
            .map(|mut init| {
                while !init.last().unwrap().iter().all(|val| *val == 0) {
                    let new = init.last().unwrap().as_slice().windows(2).fold(
                        Vec::new(),
                        |mut acc, pair| {
                            acc.push(pair[1] - pair[0]);
                            acc
                        },
                    );
                    init.push(new);
                }
                init
            })
            .map(|lists| {
                lists
                    .into_iter()
                    .fold(0isize, |acc, el| acc + el.last().unwrap())
            })
            .sum::<isize>();

    let p2 =
        input
            .lines()
            .map(|line| line.split_whitespace())
            .map(|line| line.map(|num| num.parse::<isize>().unwrap()))
            .map(|nums| nums.collect::<Vec<isize>>())
            .map(|init| vec![init])
            .map(|mut init| {
                while !init.last().unwrap().iter().all(|val| *val == 0) {
                    let new = init.last().unwrap().as_slice().windows(2).fold(
                        Vec::new(),
                        |mut acc, pair| {
                            acc.push(pair[1] - pair[0]);
                            acc
                        },
                    );
                    init.push(new);
                }
                init
            })
            .inspect(|x| println!("{x:?}"))
            .map(|lists| {
                lists
                    .into_iter()
                    .fold(0isize, |acc, el| el.first().unwrap() - acc)
            })
            .sum::<isize>();

    println!("{p1} {p2:?}");

    // dbg!(ret
    // .into_iter()
    // .fold(0isize, |acc, el| acc + el.last().unwrap()));
}
