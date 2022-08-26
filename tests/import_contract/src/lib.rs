#![no_std]
use soroban_sdk::{contractimpl, Env};

const ADD_CONTRACT_ID: [u8; 32] = [0; 32];
mod addcontract {
    soroban_sdk::contractimport!(
        file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm"
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: i32) -> i32 {
        addcontract::ContractClient::new(&env, &ADD_CONTRACT_ID).add(x, y)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{addcontract, Contract, ContractClient, ADD_CONTRACT_ID};

    #[test]
    fn test_add() {
        let e = Env::default();

        let add_contract_id = BytesN::from_array(&e, &ADD_CONTRACT_ID);
        e.register_contract_wasm(&add_contract_id, addcontract::WASM);

        let contract_id = BytesN::from_array(&e, &[1; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let x = 10i32;
        let y = 12i32;
        let z = client.add_with(x, y);
        assert!(z == 22);
    }
}