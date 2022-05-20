use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voter {
    pub balance: Meta,
    pub locking_positions: Vector<LockingPosition>,
    pub voting_power: VotePower,
    pub vote_positions: Vector<VotePosition>,
}

impl Voter {
    pub fn new(id: &VoterId) -> Self {
        Self {
            balance: 0,
            locking_positions: Vector::new(Keys::LockingPosition.as_prefix(&id).as_bytes()),
            voting_power: 0,
            vote_positions: Vector::new(Keys::VotePosition.as_prefix(&id).as_bytes()),
        }
    }
}
