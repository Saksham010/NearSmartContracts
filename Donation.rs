use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::collections::{UnorderedMap};
use near_sdk::serde::Serialize;


//Defining a cost for storage of the donation amount
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    pub beneficiary: AccountId, //Receiver of the donated money
    pub donations: UnorderedMap<AccountId,u128>, // The doner account id and amount of money the doner donated

}

//Implementing default constructor for the smart contract
impl Default for Contract{

    fn default()-> Self{
        Self{
            beneficiary: "charity.saksham010.testnet".parse().unwrap(), //charity.saksham010.testnet will get the donated money
            donations: UnorderedMap::new(b"s"), //Initializing the Near hashmap with "s" prefix
        }
    }
}

//Implementing methods for the smart contract
#[near_bindgen]
impl Contract{
    #[init]
    //Defining a parameterized constructor just incase if the deployer of the contract wants to
    #[private]
    pub fn new(beneficiary: AccountId) -> Self{
        assert!(!env::state_exists(), "Contract has already been initialized"); //If the contract has already been initialized then !env::state_exists() == false there fore throws an error: "Contract has already been initialized"
        Self{
            beneficiary,
            donations: UnorderedMap::new(b"s"),
        }
    }

    //Method to change beneficiary
    #[private]
    pub fn change_beneficiary(&mut self ,new_beneficiary: AccountId){
        self.beneficiary = new_beneficiary;
    }

    //Method to donate money via smart contract
    #[payable]
    pub fn donate(&mut self)-> U128{

        let donor: AccountId = env::predecessor_account_id(); //Near id of the caller
        let donated_amount: Balance = env::attached_deposit(); //The amount of Near sent with the function call

        //Amount of money donated so far by this doner
        let mut donated_so_far: Balance = self.donations.get(&donor).unwrap_or(0);


        //If the donor is donating for the first time then we take a small amount as storage fee
        let to_transfer_amount = if donated_so_far == 0{
            donated_amount-STORAGE_COST //Storage cost deducted
            
        }else{
            //If this is not the first time the donor is donating money and has already paid the storage cost then the attached amount is to be donated
            donated_amount
        };

        //Total amount the donor has now dontaed
        donated_so_far += donated_amount;

        //Keeping record of the donor and the amount
        self.donations.insert(&donor,&to_transfer_amount);

        //Logging the donation 
        log!("Thank you {} for donating: {}",donor.clone(),donated_amount);

        //Transferring money to the benificiary( Transfers in the next block due to sharded state)
        Promise::new(self.beneficiary.clone()).transfer(to_transfer_amount);

        U128(donated_so_far) //Returing the amount donated
    }

    //Methods for data purpose
    pub fn get_donation_for_id(&self, account: AccountId)->Donation{
        Donation{
            account_id: account.clone(),
            total_amount: U128(self.donations.get(&account).unwrap_or(0)),
        }
    }

    //Get total no of donations
    pub fn get_total_number_donations(&self)->u64{
        self.donations.len()
    }

    //Get the account id of the beneficiary
    pub fn get_name_beneficiary(&self)->AccountId{
        self.beneficiary.clone()
    }
}