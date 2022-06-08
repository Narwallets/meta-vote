use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::collections::unordered_map::UnorderedMap;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, require};

mod constants;
mod deposit;
mod internal;
mod locking_position;
mod types;
mod utils;
mod voter;
mod withdraw;
pub mod interface;

use types::*;
use utils::get_current_epoch_millis;
use voter::Voter;
use crate::utils::{days_to_millis, millis_to_days};
use crate::{constants::*, locking_position::*};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MetaVoteContract {
    pub owner_id: AccountId,
    pub voters: UnorderedMap<VoterId, Voter>,
    pub votes: UnorderedMap<ContractAddress, UnorderedMap<VotableObjId, VotingPower>>,
    pub min_locking_period: Days,
    pub max_locking_period: Days,
    pub min_deposit_amount: Meta,
    pub max_locking_positions: u8,
    pub max_voting_positions: u8,
    pub meta_token_contract_address: ContractAddress,
}

#[near_bindgen]
impl MetaVoteContract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        min_locking_period: Days,
        max_locking_period: Days,
        min_deposit_amount: Meta,
        max_locking_positions: u8,
        max_voting_positions: u8,
        meta_token_contract_address: ContractAddress,
    ) -> Self {
        // require!(!env::state_exists(), "The contract is already initialized");
        require!(min_locking_period < max_locking_period, "Review the min and max locking period");
        Self {
            owner_id,
            voters: UnorderedMap::new(Keys::Voter),
            votes: UnorderedMap::new(Keys::Votes),
            min_locking_period,
            max_locking_period,
            min_deposit_amount,
            max_locking_positions,
            max_voting_positions,
            meta_token_contract_address,
        }
    }

    // *************
    // * Unlocking *
    // *************

    pub fn unlock_position(&mut self, index: PositionIndex) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let mut locking_position = voter.get_position(index);

        let voting_power = locking_position.voting_power;
        require!(voter.voting_power >= voting_power, "Not enough free voting power to unlock!");

        log!(
            "UNLOCK: {} unlocked position {}.",
            &voter_id.to_string(),
            index
        );
        locking_position.unlocking_started_at = Some(get_current_epoch_millis());
        voter.locking_positions.replace(index, &locking_position);
        voter.voting_power -= voting_power;
        self.voters.insert(&voter_id, &voter);
    }

    pub fn unlock_partial_position(&mut self, index: PositionIndex, amount: MetaJSON) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let mut locking_position = voter.get_position(index);

        let locking_period = locking_position.locking_period;
        let amount = Meta::from(amount);

        // If the amount equals the total, then the unlock is not partial.
        if amount == locking_position.amount {
            return self.unlock_position(index);
        }
        require!(
            locking_position.amount > amount,
            "Amount too large!"
        );
        assert!(
            (locking_position.amount - amount) >= self.min_deposit_amount,
            "A locking position cannot have less than {} $META",
            self.min_deposit_amount
        );
        let remove_voting_power = self.calculate_voting_power(amount, locking_period);
        require!(
            locking_position.voting_power >= remove_voting_power
                && voter.voting_power >= remove_voting_power,
            "Not enough free voting power to unlock!"
        );

        log!(
            "UNLOCK: {} partially unlocked position {}.",
            &voter_id.to_string(),
            index
        );
        // Create a NEW unlocking position
        self.create_unlocking_position(&mut voter, amount, locking_period, remove_voting_power);

        // Decrease current locking position
        locking_position.voting_power -= remove_voting_power;
        locking_position.amount -= amount;
        voter.locking_positions.replace(index, &locking_position);

        voter.voting_power -= remove_voting_power;
        self.voters.insert(&voter_id, &voter);
    }

    // ***********
    // * Re-Lock *
    // ***********

    pub fn relock_position(
        &mut self,
        index: PositionIndex,
        locking_period: Days,
        amount_from_balance: MetaJSON
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let locking_position = voter.get_position(index);

        // Check voter balance and unlocking position amount.
        let amount_from_balance = amount_from_balance.0;
        require!(
            voter.balance >= amount_from_balance,
            "Not enough balance."
        );
        // Check if position is unlocking.
        require!(
            locking_position.unlocking_started_at.is_some(),
            "Cannot re-lock a locked position."
        );

        let now = get_current_epoch_millis();
        let unlocking_date = locking_position.unlocking_started_at.unwrap()
            + locking_position.locking_period_millis();
        
        if now < unlocking_date {
            // Position is **unlocking**.
            let remaining = unlocking_date - now;
            assert!(
                remaining < days_to_millis(locking_period),
                "The new locking period should be greater than {} days.",
                millis_to_days(remaining)
            );
        }

        log!(
            "RELOCK: {} relocked position {}.",
            &voter_id.to_string(),
            index
        );
        let amount = locking_position.amount + amount_from_balance;
        voter.remove_position(index);
        voter.balance -= amount_from_balance;
        self.deposit_locking_position(
            amount,
            locking_period,
            voter_id,
            &mut voter
        );
    }

    pub fn relock_partial_position(
        &mut self,
        index: PositionIndex,
        amount_from_position: MetaJSON,
        locking_period: Days,
        amount_from_balance: MetaJSON
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let mut locking_position = voter.get_position(index);

        // Check voter balance and unlocking position amount.
        let amount_from_balance = amount_from_balance.0;
        let amount_from_position = amount_from_position.0;
        require!(
            voter.balance >= amount_from_balance,
            "Not enough balance."
        );
        require!(
            locking_position.amount >= amount_from_position,
            "Locking position amount is not enough."
        );
        let amount = amount_from_balance + amount_from_position;
        assert!(
            amount >= self.min_deposit_amount,
            "A locking position cannot have less than {} $META.",
            self.min_deposit_amount
        );
        // Check if position is unlocking.
        require!(
            locking_position.unlocking_started_at.is_some(),
            "Cannot re-lock a locked position."
        );

        let now = get_current_epoch_millis();
        let unlocking_date = locking_position.unlocking_started_at.unwrap()
            + locking_position.locking_period_millis();

        if now < unlocking_date {
            // Position is **unlocking**.
            let remaining = unlocking_date - now;
            assert!(
                remaining < days_to_millis(locking_period),
                "The new locking period should be greater than {} days.",
                millis_to_days(remaining)
            );

            let new_amount = locking_position.amount - amount_from_position;
            assert!(
                amount >= self.min_deposit_amount,
                "A locking position cannot have less than {} $META.",
                self.min_deposit_amount
            );
            locking_position.amount = new_amount;
            voter.locking_positions.replace(index, &locking_position);
        } else {
            voter.balance += locking_position.amount - amount_from_position;
            voter.remove_position(index);
        }

        log!(
            "RELOCK: {} partially relocked position {}.",
            &voter_id.to_string(),
            index
        );
        voter.balance -= amount_from_balance;
        self.deposit_locking_position(
            amount,
            locking_period,
            voter_id,
            &mut voter
        );
    }

    pub fn relock_from_balance(
        &mut self,
        locking_period: Days,
        amount_from_balance: MetaJSON
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);

        let amount = amount_from_balance.0;
        require!(
            voter.balance >= amount,
            "Not enough balance."
        );
        assert!(
            amount >= self.min_deposit_amount,
            "A locking position cannot have less than {} $META.",
            self.min_deposit_amount
        );

        log!(
            "RELOCK: {} relocked position.",
            &voter_id.to_string()
        );
        voter.balance -= amount;
        self.deposit_locking_position(
            amount,
            locking_period,
            voter_id,
            &mut voter
        );
    }

    // ******************
    // * Clear Position *
    // ******************

    pub fn clear_locking_position(&mut self, position_index_list: Vec<PositionIndex>) {
        require!(position_index_list.len() > 0, "Index list is empty.");
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let mut position_index_list = position_index_list;

        position_index_list.sort();
        position_index_list.reverse();
        for index in position_index_list {
            let locking_position = voter.get_position(index);
            if locking_position.is_unlocked() {
                voter.balance += locking_position.amount;
                voter.remove_position(index);
            }
        }
        self.voters.insert(&voter_id, &voter);
    }

    // ************
    // * Withdraw *
    // ************

    pub fn withdraw(
        &mut self,
        position_index_list: Vec<PositionIndex>,
        amount_from_balance: MetaJSON
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let amount_from_balance = Meta::from(amount_from_balance);
        require!(voter.balance >= amount_from_balance, "Not enough balance.");
        let remaining_balance = voter.balance - amount_from_balance;

        // Clear locking positions could increase the voter balance.
        if position_index_list.len() > 0 {
            self.clear_locking_position(position_index_list);
        }

        let total_to_withdraw = voter.balance - remaining_balance;
        require!(total_to_withdraw > 0, "Nothing to withdraw.");
        voter.balance -= total_to_withdraw;

        if voter.is_empty() {
            self.voters.remove(&voter_id);
            log!("GODSPEED: {} is no longer part of Meta Vote!", &voter_id);
        } else {
            self.voters.insert(&voter_id, &voter);
        }
        self.transfer_meta_to_voter(voter_id, total_to_withdraw);
    }

    // **********
    // * Voting *
    // **********

    pub fn vote(
        &mut self,
        voting_power: VotingPowerJSON,
        contract_address: ContractAddress,
        votable_object_id: VotableObjId
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let voting_power = VotingPower::from(voting_power);
        require!(voter.voting_power >= voting_power, "Not enough available Voting Power.");

        let mut votes_for_address = voter.get_votes_for_address(&contract_address);
        let mut votes = votes_for_address.get(&votable_object_id).unwrap_or(0_u128);

        voter.voting_power -= voting_power;
        votes += voting_power;
        votes_for_address.insert(&votable_object_id, &votes);
        voter.vote_positions.insert(&contract_address, &votes_for_address);
        self.voters.insert(&voter_id, &voter);
    }

    pub fn rebalance(
        &mut self,
        voting_power: VotingPowerJSON,
        contract_address: ContractAddress,
        votable_object_id: VotableObjId
    ) {
        let voter_id = env::predecessor_account_id();
        let mut voter = self.internal_get_voter(&voter_id);
        let voting_power = VotingPower::from(voting_power);

        let mut votes_for_address = voter.get_votes_for_address(&contract_address);
        let mut votes = votes_for_address.get(&votable_object_id)
            .expect("Rebalance not allowed for unexisting Votable Object.");

        require!(votes != voting_power, "Cannot rebalance to same Voting Power.");
        if voting_power == 0 {
            return self.unvote(contract_address, votable_object_id);
        }
 
        if votes < voting_power {
            // Increase votes.
            let additional_votes = voting_power - votes;
            require!(voter.voting_power >= additional_votes, "Not enough additional Voting Power.");        
            voter.voting_power -= additional_votes;
            votes += additional_votes;
        } else {
            // Decrease votes.
            let remove_votes = votes - voting_power;
            voter.voting_power += remove_votes;
            votes -= remove_votes;
        }
        votes_for_address.insert(&votable_object_id, &votes);
        voter.vote_positions.insert(&contract_address, &votes_for_address);
        self.voters.insert(&voter_id, &voter);
    }

    pub fn unvote(
        &mut self,
        contract_address: ContractAddress,
        votable_object_id: VotableObjId
    ) {
        unimplemented!();
    }

    // ****************
    // * View Methods *
    // ****************

    pub fn get_all_locking_positions(&self) -> Vec<LockingPositionJSON> {
        let mut result = Vec::new();
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        for index in 0..voter.locking_positions.len() {
            let locking_position = voter.locking_positions.get(index)
                .expect("Locking position not found!");
            result.push(
                locking_position.to_json(Some(index))
            );
        }
        result
    }

    pub fn get_locking_position(&self, index: PositionIndex) -> Option<LockingPositionJSON> {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        match voter.locking_positions.get(index) {
            Some(locking_position) => Some(locking_position.to_json(Some(index))),
            None => None,
        }
    }

    pub fn get_balance(&self) -> BalanceJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        let balance = voter.balance + voter.sum_unlocked();
        BalanceJSON::from(balance)
    }

    pub fn get_locked_balance(&self) -> BalanceJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        BalanceJSON::from(voter.sum_locked())
    }

    pub fn get_unlocking_balance(&self) -> BalanceJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        BalanceJSON::from(voter.sum_unlocking())
    }

    pub fn get_available_voting_power(&self) -> VotingPowerJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        VotingPowerJSON::from(voter.voting_power)
    }

    pub fn get_used_voting_power(&self) -> VotingPowerJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        VotingPowerJSON::from(voter.sum_used_votes())
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;