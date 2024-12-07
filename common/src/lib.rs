use std::env;

pub fn read_path() -> String {
    let args: Vec<String> = env::args().collect();

    args[1].clone()
}
