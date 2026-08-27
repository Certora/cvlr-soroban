// Checks that `#[contractevent]` passes struct-level attributes through:
// the derives below must survive the macro for this file to compile.
// This lives outside tests/expand/ on purpose — derive expansion changes
// between rustc versions, so it must not be snapshot-tested by macrotest.

use cvlr_soroban_derive::contractevent;

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleGranted {
    #[topic]
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

    let copy = event.clone();
    assert!(event == copy);
    assert!(!format!("{copy:?}").is_empty());

    event.publish(&env);
}
