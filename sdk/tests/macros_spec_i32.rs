use std::io::Cursor;

use stellar_contract_sdk::{contractimpl, Env, IntoVal, TryFromVal};
use stellar_xdr::{ReadXdr, SpecEntry, SpecFunctionV0, SpecTypeDef};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[test]
fn test_functional() {
    let e = Env::default();
    let a = 10i32.into_val(&e);
    let b = 12i32.into_val(&e);
    let c = __add(e.clone(), a, b);
    let c = i32::try_from_val(&e, c).unwrap();
    assert_eq!(c, 22);
}

#[test]
fn test_spec() {
    let entries = SpecEntry::read_xdr(&mut Cursor::new(&__SPEC_XDR_ADD)).unwrap();
    let expect = SpecEntry::FunctionV0(SpecFunctionV0 {
        name: "add".try_into().unwrap(),
        input_types: vec![SpecTypeDef::I32, SpecTypeDef::I32].try_into().unwrap(),
        output_types: vec![SpecTypeDef::I32].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}