use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voter {
    pub balance: Meta,
    pub locking_positions: Vector<LockingPosition>,
    pub voting_power: VotePower,
    pub vote_positions: Vector<VotePosition>,

    /// Accumulative count of the locking and vote positions.
    pub locking_counter: u32,
    pub vote_counter: u32,
}

impl Voter {
    pub fn new(id: &VoterId) -> Self {
        Self {
            balance: 0,
            locking_positions: Vector::new(Keys::LockingPosition.as_prefix(&id).as_bytes()),
            voting_power: 0,
            vote_positions: Vector::new(Keys::VotePosition.as_prefix(&id).as_bytes()),
            locking_counter: 0,
            vote_counter: 0,
        }
    }
}
