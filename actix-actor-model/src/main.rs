use std::time::Duration;

use actix::prelude::*;
use actix_rt::time::sleep;

// This is our actor
struct MyActor {
    count: usize,
}

// These are the messages it can handle
struct Incr;
struct ShowCount;

impl Message for Incr {
    type Result = ();
}

impl Message for ShowCount {
    type Result = usize;
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

// Handler for `Incr` message
impl Handler<Incr> for MyActor {
    type Result = ();

    fn handle(&mut self, _msg: Incr, _ctx: &mut Context<Self>) {
        self.count += 1;
        println!("Count is now: {}", self.count)
    }
}

// Handler for `ShowCount` message
impl Handler<ShowCount> for MyActor {
    type Result = usize;

    fn handle(&mut self, _msg: ShowCount, _ctx: &mut Context<Self>) -> Self::Result {
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    let actor = MyActor { count: 0 }.start();

    actor.do_send(Incr);
    actor.do_send(Incr);
    actor.do_send(Incr);
    sleep(Duration::from_secs(2)).await;
    let res = actor.send(ShowCount).await;

    match res {
        Ok(count) => println!("COUNT: {}", count),
        Err(_) => println!("Actor communication failed"),
    }
}