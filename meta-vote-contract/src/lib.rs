use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, PromiseResult};

mod types;
mod voter;
mod vote_position;
mod locking_position;
mod constants;
use types::*;
use voter::Voter;

use crate::{constants::*, vote_position::*, locking_position::*, voter::*};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MetaVote {
    pub owner_id: AccountId,
    pub voters: UnorderedMap<VoterId, Voter>,
}
