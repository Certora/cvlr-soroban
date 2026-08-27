use soroban_sdk::Address;

#[link(wasm_import_module = "env")]
extern "C" {
    fn CERTORA_SOROBAN_is_auth(address: u64) -> u64; // should be CVT_* eventually
}

pub fn is_auth(address: Address) -> bool {
    unsafe { CERTORA_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}
