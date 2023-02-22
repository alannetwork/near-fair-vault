use crate::*;
use near_sdk::{Gas};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(300_000_000_000_000);
pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = Gas(300_000_000_000_000);


#[near_bindgen]
impl Contract {
    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        use near_sys as sys;
        assert!(env::predecessor_account_id() == self.owner_id);
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
       // const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - Gas(GAS_FOR_UPGRADE);
        unsafe {
            // Load input (new contract code) into register 0
            sys::input(0);

            //prepare self-call promise
            let promise_id =
                sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);

            //1st action, deploy/upgrade code (takes code from register 0)
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the **new code**
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                u64::from(attached_gas),
            );
        }
    }

/////////////////////METODO DE MIGRACIÖN
 /* 
UNCOMMENT IF THE CONTRACT IS ALREADY DEPLOYED

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        
        env::log_str("old state readed");


        Self {
            time_last_deposit: old_state.time_last_deposit,
            //COUNTDOWN PERIOD
            //start in 1 month
            countdown_period: old_state.countdown_period, // X amount of time 
            accountid_last_deposit: old_state.accountid_last_deposit,
            ft_token_balance: old_state.ft_token_balance,
            ft_token_id: old_state.ft_token_id,
            treasury_id: old_state.treasury_id,
            thirdparty_id: old_state.thirdparty_id,
            owner_id: old_state.owner_id,
            highest_deposit: old_state.highest_deposit,
            highest_withdraw: old_state.highest_withdraw,
            deposit_history: old_state.deposit_history,

        }
    }

*/
  

}