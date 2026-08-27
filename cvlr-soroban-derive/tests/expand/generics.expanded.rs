#![allow(dead_code)]
use cvlr_soroban_derive::contractevent;
pub struct GenericEvent<'a, T>
where
    T: Clone + Eq,
{
    pub label: &'a str,
    pub payload: T,
}
impl<'a, T> GenericEvent<'a, T>
where
    T: Clone + Eq,
{
    pub fn publish(&self, _env: &soroban_sdk::Env) {}
}
fn main() {
    let env = soroban_sdk::Env::default();
    let event = GenericEvent {
        label: "alpha",
        payload: 9_u32,
    };
    event.publish(&env);
}
