use cvlr_nondet::nondet;
use soroban_sdk::{Address, Bytes, Env, IntoVal, Map, String, Symbol, TryFromVal, Val, Vec};

pub fn nondet_address() -> Address {
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 77);
    Address::try_from_val(&Env::default(), &val).unwrap()
}

pub fn nondet_map<K, V>() -> Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 76);
    Map::try_from_val(&Env::default(), &val).unwrap()
}

pub fn nondet_string() -> String {
    let nd: u8 = nondet();
    String::from_bytes(&Env::default(), &[nd])
}

pub fn nondet_vec<V>() -> Vec<V>
where
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 75);
    Vec::try_from_val(&Env::default(), &val).unwrap()
}

pub fn nondet_symbol() -> Symbol {
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 74);
    Symbol::try_from_val(&Env::default(), &val).unwrap()
}

pub fn nondet_bytes() -> Bytes {
    let v: u64 = nondet();
    let val = Val::from_payload((v << 8) | 72);
    Bytes::try_from_val(&Env::default(), &val).unwrap()
}
