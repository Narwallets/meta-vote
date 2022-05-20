use near_sdk::BorshIntoStorageKey;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub const YOCTO_UNITS: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Keys {
    LockingPositions,
    VotePositions,
    Voters,
}

impl Keys {
	/// Creates unique prefix for collections based on a String id.
	pub fn as_prefix(&self, id: &str) -> String {
		match self {
			Keys::LockingPositions => format!("{}{}", "LP", id),
			Keys::VotePositions => format!("{}{}", "VP", id),
			Keys::Voters => format!("{}{}", "V", id),
        }
    }
}

impl BorshIntoStorageKey for Keys {}
