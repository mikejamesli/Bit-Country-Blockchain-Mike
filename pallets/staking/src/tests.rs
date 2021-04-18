#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{*};

#[test]
fn create_new_staking_pool_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(StakingModule::create_staking_pool(Origin::signed(BOB), 0, 0, 1));
    });
}