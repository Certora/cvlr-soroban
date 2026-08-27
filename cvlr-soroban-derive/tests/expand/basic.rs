#![allow(dead_code)]

use cvlr_soroban_derive::contractevent;

// no derives here so the expanded output does not depend on the
// compiler's builtin derive expansion, which changes between rustc
// versions
#[contractevent]
pub struct RoleGranted {
    // the #[allow] must survive expansion: only #[topic] is stripped
    #[topic]
    #[allow(dead_code)]
    pub role: u32,
    #[topic]
    pub account: u64,
    pub caller: bool,
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
