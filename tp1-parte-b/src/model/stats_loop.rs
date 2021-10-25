use std::{thread::sleep, time::Duration};

use actix::prelude::*;


use super::stats::{GetStats, Stats};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Loop;

pub struct StatsLoop{
    pub arbiter_stats: Addr<Stats>
}

// Provide Actor implementation for our actor
impl Actor for StatsLoop {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Stats Loops Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Stats Loops Actor is stopped");
    }
}

/// Define handler for `Loop` message
impl Handler<Loop> for StatsLoop {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, _msg: Loop, _ctx: &mut Context<Self>) -> Self::Result {
        let arbiter_stats = self.arbiter_stats.clone();
        Box::pin(async move {
            loop {
                let _result = arbiter_stats.send(GetStats).await;
                sleep(Duration::from_secs(5));
            }
        })
    }
}