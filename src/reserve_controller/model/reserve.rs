pub trait   Reserve {
    fn reserve_airline(&self, reserve: &dyn Fn(String, String) -> bool);
    fn reserve_hotel(&self, reserve: &dyn Fn(String) -> bool);
}
