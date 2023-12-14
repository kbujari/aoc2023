#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let input = include_str!("../../input/10.txt");
    let input_vec: Vec<char> = input.chars().collect();
    let line_len = input.lines().nth(0).unwrap().len() + 1; // +1 handles the \n on each line

    // TODO: Actually find the correct entrypoint rather than manually read input
    let mut idx = input.chars().position(|c| c == 'S').unwrap() + 1;
    let mut ch = input_vec[idx];
    let (mut direction, mut count) = (Direction::East, 1usize);

    // p2
    let mut idxes: Vec<usize> = Vec::new();

    while ch != 'S' {
        use Direction::*;

        (idx, direction) = match (ch, direction) {
            ('|', North) => (idx - line_len, North),
            ('|', South) => (idx + line_len, South),
            ('-', East) => (idx + 1, East),
            ('-', West) => (idx - 1, West),
            ('L', South) => (idx + 1, East),
            ('L', West) => (idx - line_len, North),
            ('J', South) => (idx - 1, West),
            ('J', East) => (idx - line_len, North),
            ('7', North) => (idx - 1, West),
            ('7', East) => (idx + line_len, South),
            ('F', North) => (idx + 1, East),
            ('F', West) => (idx + line_len, South),
            _ => panic!("invalid!"),
        };

        ch = input_vec[idx];
        count += 1;

        // p2
        idxes.push(idx);
    }

    let p1 = count / 2;

    println!("{p1}\n{idxes:?}");
}
