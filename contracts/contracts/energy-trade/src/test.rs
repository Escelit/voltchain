#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_trade() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EnergyTradeContract);
    let client = EnergyTradeContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    let prosumer = Address::generate(&env);
    let consumer = Address::generate(&env);

    let result = client.trade(&prosumer, &consumer, &10, &50);
    assert_eq!(result, symbol_short!("SUCCESS"));
}
