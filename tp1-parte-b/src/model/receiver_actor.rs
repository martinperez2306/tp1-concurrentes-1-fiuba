use actix::prelude::*;
use crate::model::reserve::{Reserve};
use crate::model::reserve_actor::{ReserveActor, ReserveMsg};
use crate::model::airline_arbiters::AirlinesArbiters;
use crate::model::hotel_ws_actor::HotelWsActor;


/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveString{
    line: String,
        arbiter_hotel: Addr<HotelWsActor>,
        arbiter_airlines: AirlinesArbiters
}

impl ReserveString {
    pub fn new(line: String, arbiter_hotel: Addr<HotelWsActor>, arbiter_airlines: AirlinesArbiters) -> ReserveString {
        ReserveString { line, arbiter_hotel, arbiter_airlines}
    }
}

// Define actor
pub struct ReceiverActor {
    reserve_actor: Addr<ReserveActor>
}

impl ReceiverActor {
    pub fn new(reserve_actor: Addr<ReserveActor>) -> ReceiverActor {
        ReceiverActor {
            reserve_actor,
        }
    }
}

// Provide Actor implementation for our actor
impl Actor for ReceiverActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("ReceiverActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("ReceiverActor is stopped");
    }
}

/// Define handler for `Reserve String` message
impl Handler<ReserveString> for ReceiverActor {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, reserve_string: ReserveString, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Reserva Recibida");
        let reserve_actor = self.reserve_actor.clone();
        Box::pin(async move {
            let reserve = build_reserve(reserve_string.line);
            let _result = reserve_actor.send(ReserveMsg::new(reserve,
                                                                  reserve_string.arbiter_hotel,
                                                                  reserve_string.arbiter_airlines)).await;
            Ok(true)
        })
    }
}

fn build_reserve(reserve_line: String) -> Reserve {
    let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
    let origin = reserve_split[0].to_string();
    let destination = reserve_split[1].to_string();
    let airline = reserve_split[2].to_string();
    let hotel = reserve_split[3].to_string();
    Reserve::new(origin, destination, airline, hotel)
}