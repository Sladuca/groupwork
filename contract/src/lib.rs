use borsh::{BorshDeserialize, BorshSerialize};
<<<<<<< HEAD
use near_bindgen::{env, near_bindgen};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
=======
use near_sdk::{env, near_bindgen, payable, Balance};
use std::collections::{HashMap};
>>>>>>> sebastien

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

<<<<<<< HEAD
#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    text: String
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: HashMap<String, String>,
}

#[near_bindgen]
impl Welcome {
    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(account_id, message);
    }

    pub fn welcome(&self, account_id: String) -> TextMessage {
        match self.records.get(&account_id) {
            None => {
                env::log(b"Using default message.");
                return TextMessage { text: format!("Hello {}", account_id) }
            },
            _ => return TextMessage { text: format!("{} {}", self.records.get(&account_id).unwrap(), account_id) }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!("howdy bob_near".to_string(), contract.welcome("bob_near".to_string()).text);
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        assert_eq!("Hello francis.near".to_string(), contract.welcome("francis.near".to_string()).text);
    }
=======

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Group {
  stake_required: Balance,
  phase: u8, // 0, 1, 2
  members: Vec<String>, // [accountId1, accountId2, ...]
  stakes: HashMap<String, Balance>, // accountId -> stake amount
  reviews: HashMap<(String, String), u8> // (reviwever, reviewee) -> rating
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GroupworkContract {
  counter: u64,
  groups: HashMap<Balance, Group>
}

#[near_bindgen]
impl GroupworkContract {

  #[payable]
  pub fn create_group(&mut self, stake_required: Balance) -> u64 {
    let mut stakes = HashMap::new();
    stakes.insert(env::signer_account_id(), env::attached_depoit());
    let mut reviews = HashMap::new();
    let group = Group {
      stake_required,
      phase: 0,
      members: vec![env::signer_account_id()],
      stakes,
      reviews
    };
    self.counter += 1;
    self.groups.insert(self.counter, group);
  }

  #[payable]
  pub fn join_group(&mut self, group_id: u64) -> Option<Group> {
    match self.groups.get_mut(group_id) {
      Some(group) => {
        // update stakes
        group.stakes.insert(env::signer_account_id(), env::attached_deposit());
        // update members
        group.members.push(value);
        group
      },
      None => {
        // TODO somehow return the funds to the caller
        None
      }
    }
  }

  pub fn get_group(&mut self, group_id: u64) -> Option<Group> {
    match self.groups.get(group_id) {
      Some(group) => { 
        // check if the signer is a memeber, return null if they arent
        if (!group.members.contains(env::signer_account_id())) {
          return None
        }
        group
      },
      None => None
    }
  }
>>>>>>> sebastien
}
