use crate::*;

pub struct VotePosition {
    amount: VotePower,
    votable_contract: String,
    votable_id: u64,
}