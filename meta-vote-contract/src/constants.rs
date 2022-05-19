use near_sdk::BorshIntoStorageKey;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Keys {
    LockingPositions,
    VotePositions,
}

impl Keys {
	/// Creates unique prefix for collections based on a String id.
	pub fn as_prefix(&self, id: &str) -> String {
		match self {
			Keys::LockingPositions => format!("{}{}", "LP", id),
			Keys::VotePositions => format!("{}{}", "VP", id),
        }
    }
}

impl BorshIntoStorageKey for Keys {}
