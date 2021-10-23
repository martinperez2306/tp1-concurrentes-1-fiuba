mod controller;
mod model;
mod webservice;
use crate::controller::reserve_controller;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    reserve_controller::process_reserves(filename.to_string());
}
