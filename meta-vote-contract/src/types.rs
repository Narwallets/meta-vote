use uint::construct_uint;
use near_sdk::{AccountId, Balance};

pub type VoterId = AccountId;
pub type VotePower = u128;
pub type Days = u16;
pub type Meta = Balance;
pub type ContractAddress = AccountId;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}
