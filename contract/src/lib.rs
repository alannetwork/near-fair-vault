// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{from_str};
use near_sdk::json_types::U128;
use near_sdk::{log,Timestamp, near_bindgen,env, Promise,Gas, require, AccountId, PanicOnDefault, PromiseOrValue, Balance};

// Define modules
pub mod external;
pub use crate::external::*;

// Define global variables

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);

// nanoseconds in a second
const NANOSECONDS: u64 = 1_000_000_000;


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    time_last_deposit: Timestamp,
    countdown_period: Timestamp,
    accountid_last_deposit: AccountId,
    ft_token_balance: Balance,
    ft_token_id: AccountId,
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
    pub fn new(accountid_last_deposit:AccountId,ft_token_id:AccountId,owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            time_last_deposit: env::block_timestamp(),
            countdown_period: 2629743, // X amount of time //COUNTDOWN PERIOD
            accountid_last_deposit,
            ft_token_balance: 0,
            ft_token_id,
            owner_id
        };
        this
    }
 
    pub fn get_time_left(&self)->u64{
        self.time_last_deposit+self.countdown_period-env::block_timestamp()
    }

    //method to transfer the ft tokens to the winner
    //ideally any one can pull the crank to send the tokens to the winner
    pub fn withdraw_winner(&mut self){

        assert!(self.time_last_deposit+self.countdown_period>=env::block_timestamp(),"The vault hasn't timed out.");

        //transfer FT tokens to winner
        ft_contract::ext(self.ft_token_id.clone())
            .with_attached_deposit(1)
            .with_static_gas(Gas(5*TGAS))
            .ft_transfer(self.accountid_last_deposit.clone(), U128::from(self.ft_token_balance.clone()), None);
        
        //update ft balance to zero (0)
        self.ft_token_balance = 0;
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

    pub fn get_vault_balance(&self) {
        self.ft_token_balance;
    }
    // Method to process bets of Fungible Tokens
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: Balance,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // 
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let deposit = amount;
        //Pick which action to execute when resolving transfer;
        match msg_json.action_to_execute.as_str() {
            "increase_deposit" => {
                //Verify that you are sending from whitelisted token contract
                assert_ne!(self.ft_token_id,env::predecessor_account_id(),"This token is not accepted.");


                //Verify that is possible to make a deposit
                //this happens when the actual date is minor to locked_until date
                //or the locked_until date hass arrived and the winner hasn't withdraw the prize

                assert!(self.time_last_deposit+self.countdown_period<=env::block_timestamp(),"The vault has timed out. Claim prize");
                
                //Verify that the deposit is on an amount of the indicated
                //In case, it reset the pending period to the case choosen
                //Put a rank between the tokens
                match amount {
                    1000000000000000000000000 => { // 1 stNEAR - 1 month
                        self.countdown_period = 2629743;
                    },
                    10000000000000000000000000 => { // 10 stNEAR - 2 weeks
                        self.countdown_period = 604800*2
                    },
                    30000000000000000000000000 => { // 30 stNEAR - 3 days
                        self.countdown_period = 86400*3;
                    },
                    50000000000000000000000000 => { // 50 stNEAR - 1 day
                        self.countdown_period = 86400;
                    },
                    100000000000000000000000000 => { // 100 stNEAR - 1 hour
                        self.countdown_period = 3600;
                    },
                    1000000000000000000000000000 => { // 1000 stNEAR - 15 mins
                        self.countdown_period = 900;
                    },
                    _ => assert!(true,"Amount not accepted."),

                }

                
                //Update available deposit
                self.ft_token_balance = self.ft_token_balance+deposit;
            
                //Update date tracker
                //Save current time
                self.time_last_deposit = env::block_timestamp();
                

                //update field of who is depositing tokens in the contract
                self.accountid_last_deposit = env::signer_account_id();
                //Log to show the history of people depositing and implement The Graph

                PromiseOrValue::Value(U128::from(0))
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
/*
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
*/


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
