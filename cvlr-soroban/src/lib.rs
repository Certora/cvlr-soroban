#![no_std]
mod auth;
mod log;
mod nondet;

/// What `#[rule]` adds to a rule's body. Not meant to be used directly.
#[doc(hidden)]
pub mod rule {
    pub use cvlr_asserts::cvlr_vacuity_check;
}

pub use auth::*;
pub use log::*;
pub use nondet::*;

pub mod testutils {}
