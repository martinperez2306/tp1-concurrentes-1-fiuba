mod model;
mod controller;

use crate::webservice_aerolineas;
use crate::webservice_hoteles;
use model::reserve::Reserve;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;
use crate::reserve_controller::controller::Controller;
use crate::reserve_controller::model::flight::Flight;
use crate::reserve_controller::model::logger;
use crate::reserve_controller::model::package::Package;

const DELAY_BETWEEN_RETRIES_SECONDS: u64 = 5;
const NO_HOTEL: &str = "-";

pub struct ReserveController {}

impl Controller for ReserveController {
    fn reserve_flight(&'static self, origin: String, destination: String, airline: String){
        let child = thread::spawn(move || self.reserve_airline(origin, destination, airline));
        child.join();
    }

    fn reserve_package(&'static self, origin: String, destination: String, airline: String, hotel: String){
        let mut children = vec![];
        children.push(thread::spawn(move || self.reserve_airline(origin, destination, airline)));
        children.push(thread::spawn(move || self.reserve_hotel(hotel)));
        for child in children {
            let _ = child.join();
        }
    }
}

impl ReserveController {
    pub fn reserve_airline(&self, origin: String, destination: String, airline: String){
        logger::log(format!("Reservando aerolinea {}", airline));
        let approved: bool = webservice_aerolineas::reservar(origin.to_string(), origin.to_string());
        if !approved {
            logger::log(format!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES_SECONDS));
            thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES_SECONDS*1000));
            logger::log(format!("Reintentando..."));
            self.reserve_airline(origin, destination, airline);
            return;
        }
        logger::log(format!("La aerolinea aprobó la reserva con origen: {} y destino: {}", origin, destination));
    }

    pub fn reserve_hotel(&self, hotel: String){
        webservice_hoteles::reservar(hotel.to_string());
        logger::log(format!("El servicio de hoteles aprobó la reserva en: {}", hotel));
    }

    pub fn process_reserve(&'static self , reserve: impl Reserve){
        reserve.process(self);
    }

    pub fn parse_reserves(&'static self, filename: &str){
        let mut children = vec![];
        if let Ok(lines) = self.read_lines(filename) {
            for reserve_line in lines.into_iter().flatten() {
                let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
                let origin = reserve_split[0].to_string();
                let destination = reserve_split[1].to_string();
                let airline = reserve_split[2].to_string();
                let hotel = reserve_split[3].to_string();
                if hotel == NO_HOTEL {
                    children.push(thread::spawn(move || self.process_reserve(Flight::new(origin, destination, airline))));
                } else {
                    children.push(thread::spawn(move || self.process_reserve(Package::new(origin, destination, airline, hotel))));
                }
            }
        }
        for child in children {
            let _ = child.join();
        }
        println!("Reserve processing finished");
    }

    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

