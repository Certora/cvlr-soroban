use cvlr_log::clog;
use soroban_sdk::Address;

pub fn clog_address(addr: &Address) {
    // lower 8 bits is tag (77), we want the upper 56 bits which is a u64
    let address = addr.to_val().get_payload() >> 8;
    clog!(address);
}