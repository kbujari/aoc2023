pub fn get_input(path: &str) -> String {
    std::fs::read_to_string(format!("input/{path}")).expect("input file not there")
}
