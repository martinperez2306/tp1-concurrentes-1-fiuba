use rand::Rng;
use std::{thread, time::Duration};


pub fn reservar(_destino: String) -> bool {
    let mut rng = rand::thread_rng();
    let seconds_to_sleep = rng.gen_range(0..10);
    thread::sleep(Duration::from_millis(seconds_to_sleep*1000));
    return true
}
