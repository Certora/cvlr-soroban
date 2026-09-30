use cvlr_nondet::{
    cvlr_nondet_i32, cvlr_nondet_small_i128, cvlr_nondet_small_u128, cvlr_nondet_u32,
    cvlr_nondet_u64, nondet, Nondet,
};
use soroban_sdk::{
    auth::Context, crypto::Hash, Address, Bytes, BytesN, Duration, Env, IntoVal, Map, String,
    Symbol, Timepoint, TryFromVal, Val, Vec, I256, U256,
};

#[cfg(not(feature = "nomuxedaddress"))]
use soroban_sdk::MuxedAddress;

pub fn nondet_val() -> Val {
    let v: u64 = nondet();
    Val::from_payload(v)
}

pub fn nondet_hash() -> Hash<32> {
    let env: Env = Env::default();
    let bytes: Bytes = nondet_val().into_val(&env);
    env.crypto().sha256(&bytes)
}

pub fn nondet_context() -> Context {
    let env: Env = Env::default();
    let val = nondet_val();
    Context::try_from_val(&env, &val).expect("nondet context")
}

pub fn nondet_address() -> Address {
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 77);
    Address::try_from_val(&Env::default(), &val).unwrap()
}

#[cfg(not(feature = "nomuxedaddress"))]
pub fn nondet_muxedaddress() -> MuxedAddress {
    MuxedAddress::from(nondet_address())
}

pub fn nondet_map_internal<K, V>(newk: fn() -> K, newv: fn() -> V) -> Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    let env = Env::default();
    let mut out: Map<K, V> = Map::new(&env);

    let mut i = 0;
    let l = nondet();
    if l <= 5 {
        while i < l {
            out.set(newk(), newv());
            i += 1;
        }
    }
    out
}

pub fn nondet_map<K, V>() -> Map<K, V>
where
    K: Nondet + IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: Nondet + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    nondet_map_internal(Nondet::nondet, Nondet::nondet)
}

pub fn nondet_map_address<V>() -> Map<Address, V>
where
    V: Nondet + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    nondet_map_internal(nondet_address, Nondet::nondet)
}

pub fn nondet_string() -> String {
    let nd: u8 = nondet();
    String::from_bytes(&Env::default(), &[nd])
}

pub fn nondet_vec_internal<V>(newv: fn() -> V) -> Vec<V>
where
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    let env = Env::default();
    let mut out: Vec<V> = Vec::new(&env);

    let mut i = 0;
    let l = nondet();
    if l <= 5 {
        while i < l {
            out.push_back(newv());
            i += 1;
        }
    }
    out
}

pub fn nondet_vec<V>() -> Vec<V>
where
    V: Nondet + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    nondet_vec_internal(Nondet::nondet)
}

pub fn nondet_vec_address() -> Vec<Address> {
    nondet_vec_internal(nondet_address)
}

pub fn nondet_vec_symbol() -> Vec<Symbol> {
    nondet_vec_internal(nondet_symbol)
}

pub fn nondet_symbol() -> Symbol {
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 74);
    Symbol::try_from_val(&Env::default(), &val).unwrap()
}

pub fn nondet_bytes1() -> Bytes {
    let v: u8 = nondet();
    Bytes::from_slice(&Env::default(), &[v])
}

pub fn nondet_bytes() -> Bytes {
    nondet_bytes1()
}

#[link(wasm_import_module = "env")]
#[allow(improper_ctypes)]
extern "C" {
    fn CVT_nondet_bytes_n_20() -> BytesN<20>;
    fn CVT_nondet_bytes_n_32() -> BytesN<32>;
    fn CVT_nondet_bytes_n_64() -> BytesN<64>;
    fn CVT_nondet_bytes_n_65() -> BytesN<65>;
}

pub fn nondet_bytesn() -> BytesN<32> {
    unsafe { CVT_nondet_bytes_n_32() }
}

/// TODO: make a macro that does the following four...
pub fn nondet_bytesn_20() -> BytesN<20> {
    unsafe { CVT_nondet_bytes_n_20() }
}

pub fn nondet_bytesn_32() -> BytesN<32> {
    unsafe { CVT_nondet_bytes_n_32() }
}

pub fn nondet_bytesn_64() -> BytesN<64> {
    unsafe { CVT_nondet_bytes_n_64() }
}

pub fn nondet_bytesn_65() -> BytesN<65> {
    unsafe { CVT_nondet_bytes_n_65() }
}

pub fn nondet_duration() -> Duration {
    Duration::from_seconds(&Env::default(), nondet())
}

pub fn nondet_timepoint() -> Timepoint {
    Timepoint::from_unix(&Env::default(), nondet())
}

pub fn nondet_u256() -> U256 {
    U256::from_parts(&Env::default(), nondet(), nondet(), nondet(), nondet())
}

pub fn nondet_i32() -> i32 {
    cvlr_nondet_i32()
}

pub fn nondet_u32() -> u32 {
    cvlr_nondet_u32()
}

pub fn nondet_u64() -> u64 {
    cvlr_nondet_u64()
}

pub fn nondet_i256() -> I256 {
    I256::from_parts(&Env::default(), nondet(), nondet(), nondet(), nondet())
}

pub fn nondet_i128() -> i128 {
    cvlr_nondet_small_i128()
}

pub fn nondet_u128() -> u128 {
    cvlr_nondet_small_u128()
}

pub fn nondet_bool() -> bool {
    cvlr_nondet_u64() > 0
}
