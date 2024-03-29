extern crate actix;

use rand::Rng;
use std::thread;
use std::time::Duration;

use actix::{Actor, Handler, Message, SyncContext};

use crate::model::logger;

#[derive(Message)]
#[rtype(result = "bool")]
pub struct ReserveFlight(pub String, pub String);

/// This actor encapsulates and simulates the call to an airline web service
pub struct AirlineWsActor {
    pub id: String,
}

impl Actor for AirlineWsActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("AirlineWs Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("AirlineWs Actor is stopped");
    }
}

impl Handler<ReserveFlight> for AirlineWsActor {
    type Result = bool;

    fn handle(
        &mut self,
        reserve: ReserveFlight,
        _ctx: &mut <AirlineWsActor as Actor>::Context,
    ) -> Self::Result {
        println!(
            "Airline: {} \n origen: {} \n destino: {}",
            self.id, reserve.0, reserve.1
        );
        let mut rng = rand::thread_rng();
        let miliseconds_to_sleep = rng.gen_range(0..10);
        thread::sleep(Duration::from_millis(miliseconds_to_sleep * 1000));
        let random_response = rng.gen_range(0..10);
        if random_response <= 5 {
            logger::log(format!(
                "Reserva de Aerolinea con origen {} y destino {} aprobada!",
                reserve.0, reserve.1
            ));
            true
        } else {
            logger::log(format!(
                "Reserva de Aerolinea con origen {} y destino {} rechazada!",
                reserve.0, reserve.1
            ));
            false
        }
    }
}
