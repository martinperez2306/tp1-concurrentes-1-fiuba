mod model;

use crate::webservice_aerolineas;
use crate::webservice_hoteles;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;
use crate::reserve_controller::model::flight::Flight;
use crate::reserve_controller::model::logger;
use crate::reserve_controller::model::package::Package;

const NO_HOTEL: &str = "-";
const DELAY_BETWEEN_RETRIES_SECONDS: u64 = 5;

pub fn reserve_airline(origin: &str, destination: &str, airline: &str){
    logger::log(format!("Reservando aerolinea {}", airline));
    let approved: bool = webservice_aerolineas::reservar(origin.to_string(), destination.to_string());
    if !approved {
        logger::log(format!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES_SECONDS));
        thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES_SECONDS*1000));
        logger::log("Reintentando...".to_string());
        reserve_airline(origin, destination, airline);
        return;
    }
    logger::log(format!("La aerolinea aprobó la reserva con origen: {} y destino: {}", origin, destination));

}

pub fn reserve_hotel(hotel: &str) {
    webservice_hoteles::reservar(hotel.to_string());
    logger::log(format!("El servicio de hoteles aprobó la reserva en: {}", hotel));
}

pub fn process_flight(flight: &Flight){
    let origin = flight.get_origin();
    let destination = flight.get_destination();
    let airline = flight.get_airline();
    let _ = thread::spawn(move || reserve_airline(&origin, &destination, &airline)).join();
}

pub fn process_package(package: &Package){
    let mut children = vec![];
    let origin = package.get_origin();
    let destination = package.get_destination();
    let airline = package.get_airline();
    let hotel = package.get_hotel();
    children.push(thread::spawn(move || reserve_airline(&origin, &destination, &airline)));
    children.push(thread::spawn(move || reserve_hotel(&hotel)));
    for child in children {
        let _ = child.join();
    }
}

pub fn parse_reserves(filename: &str){
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for reserve_line in lines.into_iter().flatten() {
            let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
            let origin = reserve_split[0].to_string();
            let destination = reserve_split[1].to_string();
            let airline = reserve_split[2].to_string();
            let hotel = reserve_split[3].to_string();
            if hotel == NO_HOTEL {
                children.push(thread::spawn(move || process_flight(&Flight::new(origin, destination, airline))));
            } else {
                children.push(thread::spawn(move || process_package(&Package::new(origin, destination, airline, hotel))));
            }
        }
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("Reserve processing finished");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
