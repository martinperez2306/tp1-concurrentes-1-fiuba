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

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Ping> for PingActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}