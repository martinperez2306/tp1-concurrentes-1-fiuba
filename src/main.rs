mod reserve_controller;
mod webservice_aerolineas;
mod webservice_hoteles;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    reserve_controller::parse_reserves(filename);
}