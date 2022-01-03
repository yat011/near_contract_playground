use std::convert::TryFrom;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::serde::Deserialize;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, ext_contract, log, env, Gas};
use near_sdk::serde::{Serialize};


/// Single swap action.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    /// Pool which should be used for swapping.
    pub pool_id: u64,
    /// Token to swap from.
    pub token_in: AccountId,
    /// Amount to exchange.
    /// If amount_in is None, it will take amount_out from previous step.
    /// Will fail if amount_in is None on the first step.
    pub amount_in: Option<U128>,
    /// Token to swap into.
    pub token_out: AccountId,
    /// Required minimum amount of token_out.
    pub min_amount_out: U128,
}

#[ext_contract(ext_swap_contract)]
pub trait SwapContract {
    #[payable]
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: Option<ValidAccountId>) -> U128 ;

    #[payable]
    fn add_liquidity(
        &mut self,
        pool_id: u64,
        amounts: Vec<U128>,
        min_amounts: Option<Vec<U128>>,
    ) ;

}



#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    swap_contract: AccountId
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new()-> Self{
        Self{
            swap_contract:  AccountId::try_from("exchange.ref-dev.testnet".to_string()).unwrap()
        }
    }

    pub fn zap(self, input_amount:u128, token_in: AccountId, token_out1:AccountId, token_out2:AccountId,  pool_id: u32){
        // let half_amount = input_amount.
        let half_amount = input_amount/2;
        log!("half amount {}", half_amount);


        let swap_action1 = SwapAction{
            pool_id: 269,
            token_in: token_in.clone(),
            token_out: token_out1.clone(),
            amount_in: Some(U128(half_amount)),
            min_amount_out: U128(0)
        } ;

        let swap_action2 = SwapAction{
            pool_id: 103,
            token_in: token_in.clone(),
            token_out: token_out2.clone(),
            amount_in: Some(U128(half_amount)),
            min_amount_out: U128(0)
        } ;

        let actions = vec![swap_action1, swap_action2];

        let refer: Option<ValidAccountId> = None;
    
        ext_swap_contract::swap(
            actions,
            refer,
            env::current_account_id(),
            0,
            Gas(110_000_000_000_000)
        );

    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{ AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn test_zap(){
        let mut contract = Contract::new();
        let token_in: AccountId = AccountId::from_str("wrap.testnet").unwrap();
        let token1: AccountId = AccountId::from_str("ref.fakes.testnet").unwrap();
        let token2: AccountId = AccountId::from_str("paras.fakes.testnet").unwrap();
        
        contract.zap(11, token_in, token1, token2, 5 );

    }

}
