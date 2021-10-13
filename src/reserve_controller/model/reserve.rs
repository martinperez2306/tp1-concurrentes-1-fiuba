use crate::reserve_controller::controller::Controller;

pub trait   Reserve {
    fn process(&self, controller: impl Controller + 'static);
}
