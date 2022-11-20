// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen,env, Promise, AccountId, PanicOnDefault};

// Define global variables

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    win_probability: u8,
    casino_edge: u8,
    minimum_bet: u128,
    owner_id: AccountId
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
    pub fn toss_coin(&mut self, coin_side:bool ) -> bool{

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

    /*
    pub fn get_payment_multiplier(&self) -> f64{
        1.94 as f64;
    }
    */
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
