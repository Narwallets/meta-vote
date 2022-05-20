use crate::{*, utils::proportional};

const MIN_LOCKING_PERIOD: Days = 30;
const MAX_LOCKING_PERIOD: Days = 300;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LockingPosition {
    pub amount: Meta,
    pub locking_period: Days,
    pub voting_power: VotePower,
}

impl LockingPosition {
    pub fn new(amount: Meta, locking_period: Days, voting_power: VotePower) -> Self {
        LockingPosition {
            amount,
            locking_period,
            voting_power,
        }
    }
}

impl MetaVoteContract {
    fn calculate_voting_power(&self, amount: Meta, locking_period: Days) -> VotePower {
        let multiplier = YOCTO_UNITS + proportional(
            4 * YOCTO_UNITS,
            (locking_period - self.min_locking_period) as u128,
            (self.max_locking_period - self.min_locking_period) as u128
        );
        proportional(amount, multiplier, YOCTO_UNITS)
    }

    pub fn create_locking_position(
        &mut self,
        amount: Meta,
        locking_period: Days,
        voter_id: VoterId
    ) {
        assert!(
            locking_period <= self.max_locking_period
                && locking_period >= self.min_locking_period,
            "Locking period must be between {} and {} days",
            self.min_locking_period, self.max_locking_period 
        );

        let mut voter = self.internal_get_voter(&voter_id);
        assert!(
            (voter.locking_positions.len() as u8) < self.max_locking_positions,
            "The max number of locking positions is {}",
            self.max_locking_positions
        );
        let locking_position = LockingPosition::new(
            amount,
            locking_period,
            self.calculate_voting_power(amount, locking_period)
        );

        voter.locking_positions.push(&locking_position);
        self.voters.insert(&voter_id, &voter);
    }
}
