#![allow(dead_code)]
use cvlr_soroban_derive::contractevent;
pub struct RoleGranted {
    #[allow(dead_code)]
    pub role: u32,
    pub account: u64,
    pub caller: bool,
}
impl RoleGranted {
    pub fn publish(&self, _env: &soroban_sdk::Env) {}
}
fn main() {
    let env = soroban_sdk::Env::default();
    let event = RoleGranted {
        role: 7,
        account: 42,
        caller: true,
    };
    event.publish(&env);
}
