#![no_std]
use cvlr_mathint::NativeInt;
use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

pub trait SorobanNativeInt {
    fn to_val(&self) -> Val;
    fn from_val(v: Val) -> NativeInt;
}

impl SorobanNativeInt for NativeInt {
    fn to_val(&self) -> Val {
        let env = Env::default();
        let my_u64: u64 = self.as_internal();
        my_u64.into_val(&env)
    }

    fn from_val(v: Val) -> NativeInt {
        let env = Env::default();
        let my_u64: u64 = u64::try_from_val(&env, &v).unwrap();
        NativeInt::new(my_u64)
    }
}
