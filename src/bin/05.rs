fn main() {
    let input = include_str!("../../input/05.txt");

    let seeds: Vec<usize> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|seed| seed.parse::<usize>().unwrap())
        })
        .collect();

    // part 2
    /* let seeds = seeds
        .chunks(2)
        .into_iter()
        .flat_map(|chunk| (chunk[0]..chunk[1]).collect::<Vec<usize>>()); */

    let x = input.split("\n\n").skip(1).map(|mapstr| {
        mapstr.lines().skip(1).map(|map| {
            let out: [usize; 3] = map
                .split_whitespace()
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            out
        })
    });

    let p1 = seeds
        .into_iter()
        .inspect(|seed| println!("Working on: {seed}"))
        .map(|init| {
            x.clone().fold(init, |mut seed, maps| {
                let found = maps
                    .into_iter()
                    .find(|[_, source, range]| seed > *source && seed < (*source + *range));
                if let Some([dest, source, _]) = found {
                    seed = dest + (seed - source);
                }
                seed
            })
        })
        .min()
        .unwrap();

    println!("{p1}");
}
