#![allow(dead_code)]

use cvlr_soroban_derive::contractevent;

// no derives here so the expanded output does not depend on the
// compiler's builtin derive expansion, which changes between rustc
// versions
#[contractevent]
pub struct GenericEvent<'a, T>
where
    T: Clone + Eq,
{
    #[topic]
    pub label: &'a str,
    pub payload: T,
}

fn main() {
    let env = soroban_sdk::Env::default();
    let event = GenericEvent {
        label: "alpha",
        payload: 9_u32,
    };

    event.publish(&env);
}
