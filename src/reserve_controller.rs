mod model;

use crate::webservice_aerolineas;
use crate::webservice_hoteles;
use model::reserve::Reserve;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use crate::reserve_controller::model::flight::Flight;
use crate::reserve_controller::model::package::Package;

const NO_HOTEL: &str = "-";

pub fn process_reserve(reserve: impl Reserve){
    reserve.reserve_airline(&webservice_aerolineas::reservar);
    reserve.reserve_hotel(&webservice_hoteles::reservar);
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
                children.push(thread::spawn(move || process_reserve(Flight::new(origin, destination, airline))));
            } else {
                children.push(thread::spawn(move || process_reserve(Package::new(origin, destination, airline, hotel))));
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
