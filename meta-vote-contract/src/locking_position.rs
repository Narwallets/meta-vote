use crate::{*, utils::proportional};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LockingPosition {
    pub amount: Meta,
    pub locking_period: Days,
    pub voting_power: VotePower,
    pub unlocking_started_at: Option<EpochMillis>,
}

impl LockingPosition {
    pub fn new(amount: Meta, locking_period: Days, voting_power: VotePower) -> Self {
        LockingPosition {
            amount,
            locking_period,
            voting_power,
            unlocking_started_at: None,
        }
    }

    pub fn to_json(&self, index: Option<u32>) -> LockingPositionJSON {
        LockingPositionJSON {
            index,
            amount: BalanceJSON::from(self.amount),
            locking_period: self.locking_period,
            voting_power: BalanceJSON::from(self.voting_power),
            unlocking_started_at: self.unlocking_started_at,
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

    pub(crate) fn create_locking_position(
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
        
        let voting_power = self.calculate_voting_power(amount, locking_period);
        let locking_position = LockingPosition::new(
            amount,
            locking_period,
            voting_power
        );

        voter.locking_positions.push(&locking_position);
        voter.voting_power += voting_power;
        self.voters.insert(&voter_id, &voter);
    }
}
