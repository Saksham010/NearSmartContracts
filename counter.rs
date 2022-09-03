use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Counter{
    count:i32
}

#[near_bindgen]
impl Counter{

    pub fn get_count(&self) -> i32{
        self.count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self){
        self.count -= 1;
    }
}