#[derive(Debug)]
enum Colors {
    Red = 0,
    Blue = 1,
    Green = 2,
}

impl std::str::FromStr for Colors {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err("asdf"),
        }
    }
}

// TODO: items are put back in the bag
fn main() {
    let input = aoc2023::get_input("02");

    let p1 = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|sets| {
            sets.split(';')
                .map(|set| {
                    set.split(',').map(|item| {
                        let mut info = item.trim().split_ascii_whitespace();

                        let num = info.next().unwrap().parse::<usize>().unwrap();
                        let color = info.next().unwrap().parse::<Colors>().unwrap();

                        (num, color)
                    })
                })
                .map(|set| {
                    let mut arr = [0usize; 3];
                    set.into_iter()
                        .for_each(|item| arr[item.1 as usize] += item.0);

                    arr
                })
        })
        .map(|sets| {
            sets.into_iter().all(|set| {
                use Colors::*;

                let b1 = set[Red as usize] <= 12;
                let b2 = set[Blue as usize] <= 14;
                let b3 = set[Green as usize] <= 13;

                b1 && b2 && b3
            })
        })
        .enumerate()
        .map(|(idx, game)| (idx + 1, game))
        .filter(|game| game.1)
        .map(|game| game.0)
        .sum::<usize>();

    let p2 = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|sets| {
            sets.split(';')
                .map(|set| {
                    set.split(',').map(|item| {
                        let mut info = item.trim().split_ascii_whitespace();

                        let num = info.next().unwrap().parse::<usize>().unwrap();
                        let color = info.next().unwrap().parse::<Colors>().unwrap();

                        (num, color)
                    })
                })
                .map(|set| {
                    let mut arr = [0usize; 3];
                    set.into_iter()
                        .for_each(|item| arr[item.1 as usize] += item.0);

                    arr
                })
        })
        .map(|sets| {
            let mut max = [0usize, 0usize, 0usize];

            sets.into_iter().for_each(|set| {
                use Colors::*;

                if set[Red as usize] > max[Red as usize] {
                    max[Red as usize] = set[Red as usize];
                }

                if set[Blue as usize] > max[Blue as usize] {
                    max[Blue as usize] = set[Blue as usize];
                }

                if set[Green as usize] > max[Green as usize] {
                    max[Green as usize] = set[Green as usize];
                }
            });

            max
        })
        .map(|set| set.into_iter().product::<usize>())
        .sum::<usize>();

    println!("{p1} {p2}");
}
