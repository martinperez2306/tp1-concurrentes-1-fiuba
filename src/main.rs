use std::env;
mod reserve_controller;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    reserve_controller::parse_reserves(filename);
}