// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{from_str};
use near_sdk::json_types::U128;
use near_sdk::collections::{UnorderedMap, Vector, LookupMap, UnorderedSet};
use near_sdk::{log,Timestamp, near_bindgen,env, Promise,Gas, require, AccountId, PanicOnDefault, PromiseOrValue, Balance};

pub use crate::external::*;
pub use crate::migrate::*;
pub use crate::governance::*;
pub use crate::views::*;
// Define modules
pub mod external;
mod migrate;
mod governance;
mod views;


// Define global variables

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);

// nanoseconds in a second
const NANOSECONDS: u64 = 1_000_000_000;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct DepositInfo {
    pub account_id: AccountId,
    pub date: Timestamp,
    pub ft_amount: String,
    pub deposit_or_withdraw: bool, //true=deposit - withdraw=false
}
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    time_last_deposit: Timestamp,
    countdown_period: Timestamp,
    accountid_last_deposit: AccountId,
    ft_token_balance: Balance,
    ft_token_id: AccountId,
    treasury_id: AccountId,
    owner_id: AccountId,
    thirdparty_id:AccountId, //meta yield account
    highest_deposit: Balance, //Highest amount somebody had deposit in the contract
    highest_withdraw: Balance, //Highest withdraw somebode had done when winning.
    max_target_amount: Balance, //Define the amount of tokens required to send the whole balance to thirdparty account (thirdparty DAO).
    deposit_history: UnorderedSet<DepositInfo>,
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
    pub fn new(accountid_last_deposit:AccountId,ft_token_id:AccountId,owner_id: AccountId,treasury_id: AccountId,thirdparty_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            time_last_deposit: env::block_timestamp(),
            //COUNTDOWN PERIOD
            //start in 1 month
            countdown_period: 2629743000000000, // X amount of time 
            accountid_last_deposit,
            ft_token_balance: 0,
            ft_token_id,
            treasury_id,
            thirdparty_id,
            owner_id,
            highest_deposit:0,
            highest_withdraw:0, 
            max_target_amount: 10000000000000000000000000000, //default target 10,000 tokens
            deposit_history:UnorderedSet::new(b"d".to_vec()),
        };
        this
    }

    //method to transfer the ft tokens to the winner
    //ideally any one can pull the crank to send the tokens to the winner
    pub fn withdraw_winner(&mut self){

        assert!(self.time_last_deposit+self.countdown_period<env::block_timestamp(),"The vault hasn't timed out.");
        let amount_being_withdrawn = self.ft_token_balance;
        let amount_to_winner = self.ft_token_balance * 49 /100;
        //transfer FT tokens to winner
        ft_contract::ext(self.ft_token_id.clone())
            .with_attached_deposit(1)
            .with_static_gas(Gas(5*TGAS))
            .ft_transfer(self.accountid_last_deposit.clone(), U128::from(amount_to_winner), None);
        log!("Deposit to winner: {}",amount_to_winner); 
        
        let amount_to_thirdparty = self.ft_token_balance * 51/100;
        //transfer FT tokens to thirdparty
        //DAO indeed
        ft_contract::ext(self.ft_token_id.clone())
            .with_attached_deposit(1)
            .with_static_gas(Gas(5*TGAS))
            .ft_transfer(self.thirdparty_id.clone(), U128::from(amount_to_thirdparty), None);
        
        //Verifity if it is the highest withdraw

        if self.highest_withdraw < self.ft_token_balance {
            self.highest_withdraw = self.ft_token_balance
        }
        //update ft balance to zero (0)
        self.ft_token_balance = 0;

        log!("New vault balance: {}",self.ft_token_balance); 
        self.countdown_period = 26297430000000000/2; //Put 15 days of new countdown period
        //Save current time
        self.time_last_deposit = env::block_timestamp();

        log!("New endtime: {}",self.get_end_date()); 

        self.deposit_history.insert(&DepositInfo{
            account_id:self.accountid_last_deposit.clone(),
            date:self.time_last_deposit,
            ft_amount:amount_being_withdrawn.to_string(),
            deposit_or_withdraw:false
        });
    }
    // Method to process bets of Fungible Tokens
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        
        // 
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let deposit = amount;
        //Pick which action to execute when resolving transfer;
        match msg_json.action_to_execute.as_str() {
            "increase_deposit" => {

                env::log_str("Processing deposit of tokens"); 
                //Verify that you are sending from whitelisted token contract
                assert_eq!(self.ft_token_id,env::predecessor_account_id(),"This token is not accepted.");


                //Verify that is possible to make a deposit
                //this happens when the actual date is minor to locked_until date
                //or the locked_until date hass arrived and the winner hasn't withdraw the prize

                assert!(self.time_last_deposit+self.countdown_period>env::block_timestamp(),"The vault has timed out. Claim prize");
                //Verify that max amount target hasn't been reached
                assert!(self.max_target_amount<amount.0,"The vault reached its max amount. Thirparty DAO is able to claim the whole vault.");



                //Verify that the deposit is on an amount of the indicated
                //In case, it reset the pending period to the case choosen
                //Put a rank between the tokens
                //Is required to turn this numbers into nanoseconds
                    if amount.0 <= 1000000000000000000000000 { // 1 stNEAR or less - 3 days
                        self.countdown_period = 86400000000000*3;
                    }else if amount.0 <=10000000000000000000000000 { // 10 stNEAR or less - 1 days
                        self.countdown_period = 86400000000000;
                    }else if amount.0 <=30000000000000000000000000 { // 30 stNEAR or less - 12 hours
                        self.countdown_period = 86400000000000/2;
                    }else if amount.0 <=50000000000000000000000000 { // 50 stNEAR or less - 3 hours
                        self.countdown_period = 86400000000000/8;
                    }else if amount.0 <1000000000000000000000000000 { // less than 1000 stNEAR - 1 hour
                        self.countdown_period = 3600000000000;
                    }else{ // 1000 stNEAR or more - 15 mins - 900000000000
                        self.countdown_period = 90000000000; // 90000000000 is 1.5 mins, so you don't wait much for testing
                    }
                log!("The new countdown period is: {}",self.countdown_period); 
                    
     
            
                //send fee FT tokens to treasury
                let covered_fees = amount.0 * 3/100;

                ft_contract::ext(self.ft_token_id.clone())
                .with_attached_deposit(1)
                .with_static_gas(Gas(5*TGAS))
                .ft_transfer(self.treasury_id.clone(), U128::from(covered_fees.clone()), None);

                log!("Deposit to fees: {}",covered_fees); 


                //Split revenue has to be done for fee
                let deposit_without_fees = amount.0 * 97 /100;
                log!("Deposit to vault: {}",deposit_without_fees);     

    
                //Update available deposit
                self.ft_token_balance = self.ft_token_balance+u128::from(deposit_without_fees);
                log!("The new vault balance is: {}",self.ft_token_balance); 
                //Update date tracker
                //Save current time
                self.time_last_deposit = env::block_timestamp();
                log!("Time last deposit: {}",self.time_last_deposit); 
                
                //calculte if this is the higgest deposit
                //If so, update the value
                if self.highest_deposit < amount.0 {
                    self.highest_deposit = amount.0;
                    log!("There is a new highest deposit: {}",self.highest_deposit); 
                }

                //update field of who is depositing tokens in the contract
                self.accountid_last_deposit = env::signer_account_id();

                log!("Account last deposit: {}",self.accountid_last_deposit); 
                //Log to show the history of people depositing and implement The Graph

                // Save history
                self.deposit_history.insert(&DepositInfo{
                    account_id:self.accountid_last_deposit.clone(),
                    date:self.time_last_deposit,
                    ft_amount:amount.0.to_string(),
                    deposit_or_withdraw:true
                });

                PromiseOrValue::Value(U128::from(0))
            }
            _ => PromiseOrValue::Value(U128::from(amount)),
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
