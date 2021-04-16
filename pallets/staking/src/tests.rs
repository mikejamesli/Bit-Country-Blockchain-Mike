#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};

fn create_new_auction_work() {
    ExtBuilder::default().build().execute_with(|| {
    });
}