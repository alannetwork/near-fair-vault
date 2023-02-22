use crate::*;
 
#[near_bindgen]
impl Contract {
 
    pub fn get_end_date(&self)->u64{
        self.time_last_deposit+self.countdown_period
    }

    pub fn get_current_timestamp(&self)->u64{
        env::block_timestamp()
    }
    //Last time somebody deposited
    // By default is the time in which the contract initialized
    pub fn get_time_last_deposit(&self)->u64{
        self.time_last_deposit
    }

    //Time left to support the vault
    pub fn get_countdown_period(&self)->u64{
        self.countdown_period
    }     
    //Get the balance of ft tokens deposited in the vault
    pub fn get_vault_balance(&self)->String {
        return self.ft_token_balance.to_string();
    }
    // Get FT contract that is accepted in the vault
    pub fn get_ft_token_id(&self)->AccountId{
        return self.ft_token_id.clone();
    }

    //Get the major amount that has been deposited in one single transaction
    //to the vault
    pub fn get_highest_deposit(&self)->Balance {
        return self.highest_deposit;
    }

    //Get highest withdraw done
    pub fn get_highest_withdraw(&self)->Balance {
        return self.highest_withdraw;
    }
    //Get the account that is getting the fee 
    //every new deposit is made
    pub fn get_treasury_id(&self)->AccountId{
        return self.treasury_id.clone();
    }

    //Get # of deposits that has been made
    pub fn get_number_deposits(&self)->u64{
        return self.deposit_history.len()

    }


    //get the max target amount
    pub fn get_max_target_amount(&self) -> Balance {
        self.max_target_amount
    }

    /// Get deposits in paginated view.
    pub fn get_list_deposits(&self, from_index: u64, limit: u64) -> Vec<DepositInfo> {
        let elements = self.deposit_history.as_vector();
        (from_index..std::cmp::min(from_index + limit, elements.len()))
            .filter_map(|index| elements.get(index))
            .collect()
    }


}
