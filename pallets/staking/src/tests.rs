#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{*};

#[test]
fn create_new_staking_pool_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(StakingModule::create_staking_pool(Origin::signed(BOB), 0, vec![1], 0,1));
        assert_eq!(StakingModule::pools(0), Some(Pool { name:vec![1], reward_percentage:10, start: 0, end: 1 }));
        assert_eq!(StakingModule::country_staking_pools(0), vec![0]);
        assert_eq!(StakingModule::pools_ended(1), vec![0]);
    });
}