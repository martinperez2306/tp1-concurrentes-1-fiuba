use actix::prelude::*;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping;

// Define actor
pub struct PingActor;

// Provide Actor implementation for our actor
impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Ping Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Ping Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Ping> for PingActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}