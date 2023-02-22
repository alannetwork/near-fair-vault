use crate::*;
 
#[near_bindgen]
impl Contract {

    //set a new max target amount
    //this target allows the thirdparty account (Thirdparty DAO) to withdraw the whole vault
    //(The vault exploded - Everyone wins!)
    pub fn set_max_target_amount(&mut self,new_max_target_amount:Balance) -> Balance {
        //if the caller is the owner
        //It can modify the parameters
        self.is_the_owner();
        self.max_target_amount=new_max_target_amount;
        self.max_target_amount
    }


    pub fn set_treasury(&mut self,new_treasury_id:AccountId) -> AccountId {
        //if the caller is the owner
        //It can modify the parameters
        self.is_the_owner();
        self.treasury_id=new_treasury_id;
        self.treasury_id.clone()
    }


     //validate if the owner is the caller
     #[private]
    pub fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
    }


}
