extern crate actix;

use std::{thread};
use std::time::{Duration};
use rand::Rng;

use actix::{Actor, Handler, Message, SyncContext};

use crate::model::logger;


#[derive(Message)]
#[rtype(result = "bool")]
pub struct ReserveHotel(pub String);

/// This actor encapsulates and simulates the call to a hotel web service
pub struct HotelWsActor {
    pub id: String
}

impl Actor for HotelWsActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("HotelWs Actor is alive");
     }
 
     fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("HotelWs Actor is stopped");
     }
}

impl Handler<ReserveHotel> for HotelWsActor {
    type Result = bool;

    fn handle(&mut self, reserve: ReserveHotel, _ctx: &mut <HotelWsActor as Actor>::Context) -> Self::Result  {
        println!("Hotel: {} \n destino: {}", self.id, reserve.0);
        let mut rng = rand::thread_rng();
        let miliseconds_to_sleep = rng.gen_range(0..10);
        thread::sleep(Duration::from_millis(miliseconds_to_sleep * 1000));
        logger::log(format!("Reserva de Hotel con destino {} aprobada!", reserve.0));
        true
    }
}
