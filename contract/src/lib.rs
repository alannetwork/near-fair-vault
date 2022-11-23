// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{from_str};
use near_sdk::json_types::U128;
use near_sdk::{log, near_bindgen,env, Promise,Gas, require, AccountId, PanicOnDefault, PromiseOrValue, Balance};

// Define modules
pub mod external;
pub use crate::external::*;

// Define global variables

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    win_probability: u8,
    casino_edge: u8,
    minimum_bet: u128,
    owner_id: AccountId
}

// Have to repeat the same trait for our own implementation.
trait ValueReturnTrait {
    fn ft_toss_coin(&self,bet: U128, coin_side_choosen:bool) -> PromiseOrValue<U128>;
}

/// This is format of output via JSON for the auction message.
#[derive( Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
     
    pub action_to_execute: String,
    pub coin_side_choosen: bool,
   
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default data and the owner ID
        that's passed in
    */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            win_probability: 128, // ~50%
            casino_edge: 35, // ~3.5 basis points
            minimum_bet: 100000000000000000000000, 
            owner_id
        };
        this
    }

     // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    #[payable]
    pub fn near_toss_coin(&mut self, coin_side:bool ) -> bool{

        let account_id = env::signer_account_id();
        let bet = env::attached_deposit();
        log!("Bet {}", bet);
        assert!(bet>=self.minimum_bet,"Minimum bet is not achieved.");
        

        let amount_to_pay:u128= (bet as f64*1.94) as u128;
        let contract_balance = env::account_balance()-bet-env::account_locked_balance();

        log!("Contract balance {}", contract_balance);
        log!("Amount to pay if win {}", amount_to_pay);
        
        assert!(amount_to_pay<contract_balance,"Contract doesn't have enough balance to pay this bet, try with a lower bet");

        env::log_str("Coin is flipping");  
        let toss_result = self.get_coin_side();

        if coin_side == toss_result {
            log!("¡You win! Paying bet {}", amount_to_pay);
            Promise::new(account_id).transfer(amount_to_pay as u128);
            return true;
        }
        return false;
    }

    pub fn get_coin_side(&self) -> bool{
        let rand: u8 = *env::random_seed().get(0).unwrap();
        return rand < self.win_probability;
    }


    pub fn withdraw_owner(&self){
        self.is_the_owner();
        let contract_balance = env::account_balance()-env::account_locked_balance();
        let amount_to_withdraw = contract_balance/2;
        Promise::new(env::predecessor_account_id()).transfer(amount_to_withdraw as u128);
    }
    
    //validate if the owner is the caller
    #[private]
    pub fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "You are not the contract owner."
        );
    }    

    // Method to process bets of Fungible Tokens
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {

        let msg_json: MsgInput = from_str(&msg).unwrap();
        let bet = amount;
        let coin_side_choosen = msg_json.coin_side_choosen;
        match msg_json.action_to_execute.as_str() {
            "toss-coin" => {

                assert!(bet>=U128::from(self.minimum_bet),"Minimum bet is not achieved.");
                let mut amount_to_pay:u128= (u128::from(bet) as f64*1.94) as u128;

                log!("Amount to pay, in case of win = {}", amount_to_pay);

                // Measure how much tokens does the contract have.
                // assert!(amount_to_pay<contract_balance,"Contract doesn't have enough balance to pay this bet, try with a lower bet");

                // Request result from seed
                // An oracle can improve this
                env::log_str("Coin is flipping");  
                let toss_result = self.get_coin_side();
                //let mut amount:u128 = "0".parse().expect("Not an integer");
                if coin_side_choosen == toss_result {
                    log!("¡You win! Paying bet {} tokens", amount_to_pay);
                    //amount = amount_to_pay;

                    //XCC to transfer FT tokens to new account
                    // Create a promise to call ft_transfer of FT contract
                    let ft_contract_account:AccountId = env::predecessor_account_id();
                    let signer_account_id:AccountId = env::signer_account_id();
                    ft_contract::ext(ft_contract_account)
                        .with_attached_deposit(1)
                        .with_static_gas(Gas(5*TGAS))
                        .ft_transfer(signer_account_id,U128::from(amount_to_pay), None);

                    PromiseOrValue::Value(U128::from(0))
                }else{
                    //amount = amount_to_pay;
                    amount_to_pay = "0".parse().expect("Not an integer");
                    log!("¡You Lost! {} tokens removed from your account", u128::from(bet));
                    log!("Amount to pay {}", amount_to_pay);
                    PromiseOrValue::Value(U128::from(0))

                }

                /*let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                Self::ext(account_id)
                    .with_static_gas(prepaid_gas - GAS_FOR_FT_ON_TRANSFER)
                    .ft_toss_coin(amount,coin_side_choosen)
                    .into()*/
            }
            _ => PromiseOrValue::Value(U128::from(amount)),
        }
    }



    /*
    pub fn get_payment_multiplier(&self) -> f64{
        1.94 as f64;
    }
    */
}

#[near_bindgen]
impl ValueReturnTrait for Contract {
    fn ft_toss_coin(&self,bet: U128, coin_side_choosen:bool) -> PromiseOrValue<U128> {
        assert!(bet>=U128::from(self.minimum_bet),"Minimum bet is not achieved.");
        let mut amount_to_pay:u128= (u128::from(bet) as f64*1.94) as u128;

        log!("Amount to pay, in case of win = {}", amount_to_pay);

        // Measure how much tokens does the contract have.
        // assert!(amount_to_pay<contract_balance,"Contract doesn't have enough balance to pay this bet, try with a lower bet");

        // Request result from seed
        // An oracle can improve this
        env::log_str("Coin is flipping");  
        let toss_result = self.get_coin_side();
        let mut amount:u128 = "0".parse().expect("Not an integer");
        if coin_side_choosen == toss_result {
            log!("¡You win! Paying bet {}", amount_to_pay);
            //amount = amount_to_pay;
            PromiseOrValue::Value(U128::from(0))
        }else{
            //amount = amount_to_pay;
            amount_to_pay = "0".parse().expect("Not an integer");
            log!("¡You Lost! {} tokens removed from your account", u128::from(bet));
            log!("Amount to pay {}", amount_to_pay);
            PromiseOrValue::Value(U128::from(0))

        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_token_bet() {
    }

    #[test]
    fn ft_token_bet() {
    }
}
