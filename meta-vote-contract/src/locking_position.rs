use crate::*;

const MIN_LOCKING_PERIOD: Days = 30;
const MAX_LOCKING_PERIOD: Days = 300;

pub struct LockingPosition {
    pub amount: Meta,
    pub locking_period: Days,
    pub voting_power: VotePower,
}

impl LockingPosition {
    pub fn new(amount: Meta, locking_period: Days) -> Self {
        assert!(locking_period >= MIN_LOCKING_PERIOD && locking_period <= MAX_LOCKING_PERIOD);
        LockingPosition {
            amount,
            locking_period,
            voting_power: calculate_voting_power(amount, locking_period),
        }
    }
}

fn calculate_voting_power(amount: Meta, locking_period: Days) -> VotePower {
    let multiplier = get_multiplier(locking_period);
    (amount as f64 * multiplier).trunc() as VotePower
}

fn get_multiplier(locking_period: Days) -> f64 {
    1 as f64
        + 4 as f64
        * ((locking_period as f64 - MIN_LOCKING_PERIOD as f64)
            / (MAX_LOCKING_PERIOD as f64 - MIN_LOCKING_PERIOD as f64))
}