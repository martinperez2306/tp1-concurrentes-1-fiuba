use actix::prelude::*;
use crate::model::reserve::Reserve;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveString{
    line: String
}

impl ReserveString {
    pub fn new(line: String) -> ReserveString {
        ReserveString { line }
    }
}

// Define actor
pub struct ReceiverActor;

// Provide Actor implementation for our actor
impl Actor for ReceiverActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }
}

/// Define handler for `Reserve String` message
impl Handler<ReserveString> for ReceiverActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, reserve_string: ReserveString, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Reserve received");
        let reserve = build_reserve(reserve_string.line);
        Ok(true)
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