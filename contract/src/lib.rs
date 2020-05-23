use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance, Promise};
use std::collections::{HashMap, HashSet};

const REPUTATION_MODIFIER: u128 = 1_000_000_000;

pub type Reputation = Balance;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Default, Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct Group {
  stake_required: Balance,
  total_stake: Balance,
  members: HashSet<String>, // [accountId1, accountId2, ...]
  stakes: HashMap<String, Balance>, // accountId -> stake amount
  ratings: HashMap<(String, String), u8> // (reviwever, reviewee) -> rating
}

impl Group {

  fn check_if_done(& self) -> bool {
    let members = self.members.iter();
    let product = members.clone().flat_map(|reviewee| 
      members.clone().map(move |reviewer| (reviewer.clone(), reviewee.clone()))
    );
    product.fold(true, |acc, k| if acc { self.ratings.contains_key(&k) } else { false })
  }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GroupworkContract {
  counter: u64,
  reputations: HashMap<String, Reputation>, // accountId -> reputation
  groups: HashMap<u64, Group>
}

#[near_bindgen]
impl GroupworkContract {

  fn update_reputations(&mut self, group_id: u64) {
    match self.groups.get(&group_id) {
      Some(group) => {
        let reps = group.ratings.iter()
          .map(|((_, reviewee), rating)| (reviewee.clone(), u128::from(*rating) * REPUTATION_MODIFIER));
        for (reviewee, reputation) in reps {
          self.reputations.insert(reviewee, reputation);
        }
      },
      None => {}
    };
  }

  #[payable]
  pub fn create_group(&mut self, stake_required: Balance) -> u64 {
    let mut stakes = HashMap::new();
    stakes.insert(env::signer_account_id(), env::attached_deposit());
    let ratings = HashMap::new();
    let mut members = HashSet::new();
    members.insert(env::signer_account_id());
    let group = Group {
      stake_required,
      total_stake: env::attached_deposit(),
      members,
      stakes,
      ratings
    };
    self.counter += 1;
    self.groups.insert(self.counter, group);
    self.counter
  }

  /// join a group with given id and return the group if it successful, None if not
  #[payable]
  pub fn join_group(&mut self, group_id: u64) -> Option<Group> {
    match self.groups.get_mut(&group_id) {
      Some(group) => {
        // if they didn't pay enough, give their money back and return None
        if env::attached_deposit() < group.stake_required {
          Promise::new(env::predecessor_account_id()).transfer(env::attached_deposit());
          return None;
        }
        // update stakes
        group.stakes.insert(env::signer_account_id(), env::attached_deposit());
        group.total_stake += env::attached_deposit();
        // update members
        group.members.insert(env::signer_account_id());
        Some(group.clone())
      },
      None => {
        // group DNE, give the caller their funds back and return None
        Promise::new(env::predecessor_account_id()).transfer(env::attached_deposit());
        None
      }
    }
  }

  /// gets and returns the group struct if the caller is a member, otherwise returns None
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

  /// returns true if successful, false if not
  pub fn submit_rating(&mut self, group_id: u64, other_account_id: String, rating: u8) -> bool {
    if rating > 5 {
      false
    } else {
      // get the group, or None if DNE
      match self.groups.get_mut(&group_id) {
        Some(group) => {
          let k = (env::signer_account_id(), other_account_id.clone());
          // return none if they already rated this person
          if group.ratings.contains_key(&k) {
            false
          } else {
            group.ratings.insert(k, rating);
            true
          }
        },
        None => false
      }
    }
  }

  pub fn finalize_group(&mut self, group_id: u64) -> bool {
    self.update_reputations(group_id);
    match self.groups.get_mut(&group_id) {
      Some(group) => {
        if group.check_if_done() {
          let total_reputation = self.reputations.iter()
            .filter_map(|(account_id, reputation)| if group.members.contains(account_id) { Some(reputation) } else { None })
            .fold(0u128, |acc, curr| acc + curr);
    
          let weights: HashMap<String, Reputation> = self.reputations.iter()
            .filter(|&(account_id, _)| group.members.contains(account_id))
            .map(|(account_id, reputation)| (account_id.to_owned(), *reputation / total_reputation))
            .collect();
    
          for (account_id, stake) in group.stakes.iter_mut() {
            *stake = *weights.get(account_id).unwrap();
          };
          true
        } else {
          false
        }
      },
      None => false
    }
  }
}
