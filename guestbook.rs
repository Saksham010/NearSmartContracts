use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector};
use near_sdk::{near_bindgen,serde_json};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    messages: Vector<String>,
}

impl Default for Contract{
    fn default()->Self{
        Self{
            messages : Vector::new(b"v") //Added prefix
        }
    }
}

#[near_bindgen]
impl Contract{
    pub fn add_message(&mut self, text:String){
        self.messages.push(&text);
    }

    pub fn get_messages(&self)-> String{
        let mut vectorStorage = vec![];
        
        //Number of message in the vector
        let length = self.messages.len();

        for i in 0..length{
            let element = self.messages.get(i);
            vectorStorage.push(element);
        }

        return serde_json::to_string(&vectorStorage).unwrap();
       
    }
}

