pub fn get_input(day: &str) -> String {
    std::fs::read_to_string(format!("input/{day}.txt")).expect("input file not there")
}
