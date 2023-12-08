fn main() {
    let input = include_str!("../../input/06.txt")
        .lines()
        .map(|line| line.split_whitespace().skip(1))
        .map(|line| line.map(|item| item.parse::<usize>().unwrap()));

    let input = input
        .clone()
        .next()
        .unwrap()
        .zip(input.clone().nth(1).unwrap());

    let p1 = input
        .into_iter()
        .map(|(time, distance)| {
            (1..time)
                .map(|windup| windup * (time - windup))
                .filter(|dist| *dist > distance)
                .count()
        })
        .product::<usize>();

    let input2: [usize; 2] = include_str!("../../input/06.txt")
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_whitespace().collect::<String>())
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let p2 = (1..input2[0])
        .map(|windup| windup * (input2[0] - windup))
        .filter(|dist| *dist > input2[1])
        .count();

    println!("{p1} {p2}");
}
