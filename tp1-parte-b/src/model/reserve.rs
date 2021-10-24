use actix::prelude::*;
use crate::model::receiver_actor::ReceiverActor;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveMessage {
    reserve: Reserve,
}
pub struct Reserve {
    origin: String,
    destination: String,
    airline: String,
    hotel: String,
}

impl ReserveMessage{
    pub fn new(reserve: Reserve) -> ReserveMessage {
        ReserveMessage {
            reserve,
        }
    }
    pub fn get_reserve(&self) -> Reserve {
        self.reserve.clone()
    }
}

impl Reserve {
    pub fn new(origin: String, destination: String, airline: String, hotel: String) -> Reserve {
        Reserve {
            origin,
            destination,
            airline,
            hotel
        }
    }
    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }
    pub fn get_destination(&self) -> String {
        self.destination.clone()
    }
    pub fn get_airline(&self) -> String {
        self.airline.clone()
    }
    pub fn get_hotel(&self) -> String {
        self.hotel.clone()
    }
    pub fn clone(&self) -> Reserve {
        Reserve::new(self.origin.clone(), self.destination.clone(), self.airline.clone(), self.hotel.clone())
    }
}