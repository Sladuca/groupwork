use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Balance};
use std::collections::{HashMap};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Default, Clone, BorshDeserialize, BorshSerialize)]
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
  groups: HashMap<u64, Group>
}

#[near_bindgen]
impl GroupworkContract {

  #[payable]
  pub fn create_group(&mut self, stake_required: Balance) -> u64 {
    let mut stakes = HashMap::new();
    stakes.insert(env::signer_account_id(), env::attached_deposit());
    let reviews = HashMap::new();
    let group = Group {
      stake_required,
      phase: 0,
      members: vec![env::signer_account_id()],
      stakes,
      reviews
    };
    self.counter += 1;
    self.groups.insert(self.counter, group);
    self.counter
  }

  #[payable]
  pub fn join_group(&mut self, group_id: u64) -> Option<Group> {
    match self.groups.get_mut(&group_id) {
      Some(group) => {
        // update stakes
        group.stakes.insert(env::signer_account_id(), env::attached_deposit());
        // update members
        group.members.push(env::signer_account_id());
        Some(group.clone())
      },
      None => {
        // TODO somehow return the funds to the caller
        None
      }
    }
  }

  pub fn get_group(&mut self, group_id: u64) -> Option<Group> {
    match self.groups.get(&group_id) {
      Some(group) => { 
        // check if the signer is a memeber, return null if they arent
        if !group.members.contains(&env::signer_account_id()) {
          return None
        }
        Some(group.clone())
      },
      None => None
    }
  }
}
