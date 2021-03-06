use uint::construct_uint;
use near_sdk::{AccountId, Balance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;

pub type VoterId = AccountId;
pub type VoterIdJSON = String;
pub type VotingPower = u128;
pub type Days = u16;
pub type Meta = Balance;
pub type ContractAddress = AccountId;
pub type VotableObjId = String;
pub type EpochMillis = u64;
pub type PositionIndex = u64;

pub type BalanceJSON = U128;
pub type MetaJSON = U128;
pub type VotingPowerJSON = U128;
pub type ContractAddressJSON = String;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LockingPositionJSON {
    pub index: Option<PositionIndex>,
    pub amount: MetaJSON,
    pub locking_period: Days,
    pub voting_power: VotingPowerJSON,
    pub unlocking_started_at: Option<EpochMillis>,
    pub is_unlocked: bool,
    pub is_unlocking: bool,
    pub is_locked: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VotableObjectJSON {
    pub votable_contract: ContractAddressJSON,
    pub id: VotableObjId,
    pub current_votes: VotingPowerJSON
}
