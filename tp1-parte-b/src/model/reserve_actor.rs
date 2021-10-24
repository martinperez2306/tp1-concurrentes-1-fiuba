use actix::prelude::*;
use crate::model::reserve::Reserve;

use super::{process_finished::ProcessFinished, reserve::ReserveMessage};

const NO_HOTEL: &str = "-";

// Define actor
pub struct ReserveActor;

// Provide Actor implementation for our actor
impl Actor for ReserveActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("ReserveActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("ReserveActor is stopped");
    }
}

/// Define handler for `Reserve` message
impl Handler<ReserveMessage> for ReserveActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: ReserveMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let reserve = msg.get_reserve();
        let caller = msg.get_caller();
        process_reserve(reserve);
        let result = caller.send(ProcessFinished {result: "TERMINE".to_string()});
        Ok(true)
    }
}

fn process_reserve(reserve: Reserve) {
    let origin = reserve.get_origin();
    let destination = reserve.get_destination();
    let airline = reserve.get_airline();
    let hotel = reserve.get_hotel();
    if hotel == NO_HOTEL {
        println!("Procesar Vuelo con Origen {}, Destino {} y Aerolinea {}", origin, destination, airline);
    } else {
        println!("Procesar Paquete con Origen {}, Destino {}, Aerolinea {} y Hotel {}", origin, destination, airline, hotel);
    }
}