use crate::reserve_controller::model::DELAY_BETWEEN_RETRIES_SECONDS;
use crate::reserve_controller::model::reserve::Reserve;
use std::{thread, time::Duration};
use crate::reserve_controller::model::logger;

pub struct Package {
    origin: String,
    destination: String,
    airline: String,
    hotel: String
}

impl Package {
    pub fn new(origin: String, destination: String, airline: String, hotel: String) -> Package {
        Package{ origin, destination, airline, hotel }
    }
}

impl Reserve for Package {
    fn reserve_airline(&self, reserve: &dyn Fn(String, String) -> bool){
        logger::log(format!("Reservando aerolinea {}", self.airline));
        let approved: bool = reserve(self.origin.to_string(), self.origin.to_string());
        if !approved {
            logger::log(format!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES_SECONDS));
            thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES_SECONDS*1000));
            logger::log(format!("Reintentando..."));
            self.reserve_airline(reserve);
            return;
        }
        logger::log(format!("La aerolinea aprobó la reserva con origen: {} y destino: {}", self.origin, self.destination));
    }

    fn reserve_hotel(&self, reserve: &dyn Fn(String) -> bool) {
        reserve(self.hotel.to_string());
        logger::log(format!("El servicio de hoteles aprobó la reserva en: {}", self.hotel));
    }
}
