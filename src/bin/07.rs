use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    Joker,
    Number(u32),
    // Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

/* impl std::str::FromStr for Hand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {}
} */

fn main() {
    let mut input = include_str!("../../input/07.txt")
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|lvec| {
            let cards: [Card; 5] = lvec[0]
                .chars()
                .into_iter()
                .map(|ch| match ch {
                    'T' => Card::Number(10),
                    // 'J' => Card::Jack,
                    'J' => Card::Joker,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => Card::Number(ch.to_digit(10).unwrap()),
                })
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();

            let mut counts = HashMap::<Card, u32>::new();

            cards
                .iter()
                .for_each(|card| *counts.entry(*card).or_default() += 1);

            // p2 only
            let jokers = counts.insert(Card::Joker, 0).unwrap_or(0);

            let mut sorted: Vec<u32> = counts.drain().map(|(_, v)| v).collect();
            sorted.sort_unstable_by(|a, b| Ord::cmp(&b, &a));

            // p2 only
            sorted[0] += jokers;

            let hand = match sorted.as_slice() {
                [5, ..] => Hand::FiveOfAKind(cards),
                [4, ..] => Hand::FourOfAKind(cards),
                [3, 2, ..] => Hand::FullHouse(cards),
                [3, ..] => Hand::ThreeOfAKind(cards),
                [2, 2, ..] => Hand::TwoPair(cards),
                [2, ..] => Hand::OnePair(cards),
                [1, ..] => Hand::HighCard(cards),
                _ => panic!("Couldn't build hand from {cards:?}"),
            };

            (hand, lvec[1].parse::<usize>().unwrap())
        })
        .collect::<Vec<(Hand, usize)>>();

    input.sort_unstable();

    let p1 = input.iter().enumerate().map(|(idx, (_, bid))| (idx + 1) * bid).sum::<usize>();

    println!("{p1}");
}
