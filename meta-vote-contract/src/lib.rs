use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseResult};

mod constants;
mod deposit;
mod interface;
mod internal;
mod locking_position;
mod types;
mod utils;
mod vote_position;
mod voter;
use types::*;
use voter::Voter;

use crate::{constants::*, vote_position::*, locking_position::*, voter::*};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MetaVoteContract {
    pub owner_id: AccountId,
    pub voters: UnorderedMap<VoterId, Voter>,
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
        // assert!(!env::state_exists(), "The contract is already initialized");
        assert!(min_locking_period < max_locking_period, "Review the min and max locking period");
        Self {
            owner_id,
            voters: UnorderedMap::new(Keys::Voter),
            min_locking_period,
            max_locking_period,
            min_deposit_amount,
            max_locking_positions,
            max_voting_positions,
            meta_token_contract_address,
        }
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
                locking_position.to_json(Some(index as u32))
            );
        }
        result
    }

    pub fn get_locking_position(&self, index: u32) -> Option<LockingPositionJSON> {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        match voter.locking_positions.get(index as u64) {
            Some(locking_position) => Some(locking_position.to_json(Some(index))),
            None => None,
        }
    }

    pub fn get_balance(&self) -> BalanceJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        BalanceJSON::from(voter.balance)
    }

    pub fn get_locked_balance(&self) -> BalanceJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        BalanceJSON::from(voter.sum_locked())
    }

    pub fn get_available_voting_power(&self) -> VotePowerJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        VotePowerJSON::from(voter.voting_power)
    }

    pub fn get_used_voting_power(&self) -> VotePowerJSON {
        let voter_id = env::predecessor_account_id();
        let voter = self.internal_get_voter(&voter_id);
        VotePowerJSON::from(voter.sum_used_votes())
    }


}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};
    use near_sdk::json_types::{ValidAccountId, U128};
    mod utils;
    use utils::*;
    use super::*;

    fn basic_context() -> VMContext {
        get_context(
            meta_token_account(),
            ntoy(TEST_INITIAL_BALANCE),
            0,
            to_ts(GENESIS_TIME_IN_DAYS),
            false,
        )
    }

    fn new_contract() -> MetaVoteContract {
        MetaVoteContract::new(
            owner_account(),
            MIN_LOCKING_PERIOD,
            MAX_LOCKING_PERIOD,
            MIN_DEPOSIT_AMOUNT,
            MAX_LOCKING_POSITIONS,
            MAX_VOTING_POSITIONS,
            meta_token_account(),
        )
    }

    fn get_contract_setup(context: VMContext) -> MetaVoteContract {
        testing_env!(context.clone());
        let contract = new_contract();
        contract
    }

    #[test]
    fn test_single_deposit() {
        let context = get_context(
            meta_token_account(),
            ntoy(TEST_INITIAL_BALANCE),
            0,
            to_ts(GENESIS_TIME_IN_DAYS),
            false,
        );
        let mut contract = get_contract_setup(context);

        let sender_id: ValidAccountId = voter_account().try_into().unwrap();
        let amount: U128 = U128::from(2 * YOCTO_UNITS);
        let msg: String = "30".to_owned();

        contract.ft_on_transfer(sender_id.clone(), amount.clone(), msg.clone());
        assert_eq!(1, contract.voters.len(), "Voter was not created!");

        let voter = contract.internal_get_voter(&sender_id.to_string());
        assert_eq!(1, voter.locking_positions.len(), "Locking position was not created!");

        let vote_power = contract.calculate_voting_power(
            Meta::from(amount),
            msg.parse::<Days>().unwrap()
        );
        assert_eq!(vote_power, voter.voting_power, "Incorrect voting power calculation!");
    }
}
