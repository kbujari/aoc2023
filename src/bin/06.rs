fn main() {
    let input = "Time:      7  15   30
Distance:  9  40  200" /* include_str!("../../input/06.txt") */
        .lines()
        .map(|line| line.split_whitespace().skip(1))
        .map(|line| line.map(|item| item.parse::<usize>().unwrap()));

    let input = input
        .clone()
        .nth(0)
        .unwrap()
        .zip(input.clone().nth(1).unwrap());

    let p1 = input
        .into_iter()
        .map(|(time, distance)| time)
        .product::<usize>();

    println!("{p1:?}");
}
