use actix::prelude::*;
use crate::model::receiver_actor::ReceiverActor;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveMessage {
    reserve: Reserve,
    caller_addr: Addr<ReceiverActor>
}
pub struct Reserve {
    origin: String,
    destination: String,
    airline: String,
    hotel: String,
}

impl ReserveMessage{
    pub fn new(reserve: Reserve, caller_addr: Addr<ReceiverActor>) -> ReserveMessage {
        ReserveMessage {
            reserve,
            caller_addr,
        }
    }
    pub fn get_reserve(&self) -> Reserve {
        self.reserve.clone()
    }
    pub fn get_caller(&self) -> Addr<ReceiverActor> {
        self.caller_addr.clone()
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