fn to_numchar(s: &str) -> char {
    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '1',
    }
}

fn main() {
    let input = aoc2023::get_input("input/01.txt");

    let p1 = input
        .lines()
        .map(|el| {
            let first = el.chars().find(char::is_ascii_digit).unwrap();
            let last = el.chars().rev().find(char::is_ascii_digit).unwrap();

            (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap()
        })
        .sum::<u32>();

    let p2 = input
        .lines()
        .map(|el| {
            let list = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            let first_char = el
                .char_indices()
                .find(|(_, ch)| ch.is_ascii_digit())
                .unwrap();

            let first_word = list
                .iter()
                .map(|num| (el.find(num).unwrap_or(100), num))
                .min()
                .unwrap();

            let first = if first_char.0 < first_word.0 {
                first_char.1
            } else {
                to_numchar(first_word.1)
            };

            let last_char = el
                .char_indices()
                .rev()
                .find(|(_, ch)| ch.is_ascii_digit())
                .unwrap();

            let last_word = list
                .iter()
                .map(|num| (el.match_indices(num).last(), num))
                .max()
                .unwrap();

            let last = match last_word.0 {
                None => last_char.1,
                Some(val) => {
                    if val.0 > last_char.0 {
                        to_numchar(last_word.1)
                    } else {
                        last_char.1
                    }
                }
            };

            (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap()
        })
        .sum::<u32>();

    println!("{p1} {p2}");
}
