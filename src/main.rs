mod reserve_controller;
mod webservice_aerolineas;
mod webservice_hoteles;
use std::env;
use reserve_controller::ReserveController;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let reserve_controller: ReserveController = ReserveController{};
    reserve_controller.parse_reserves(filename);
}