#![cfg(test)]

use super::*;

use crate as auction;
use frame_support::{
    construct_runtime, impl_outer_event, impl_outer_origin, impl_outer_dispatch, parameter_types, traits::EnsureOrigin, weights::Weight,
};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::IdentityLookup;
use primitives::{CurrencyId, Amount, BlockNumber};
use orml_nft;
use pallet_nft as NFTModule;

parameter_types! {
    pub const BlockHashCount: u32 = 256;
}

pub type AccountId = u128;
pub type AuctionId = u64;
pub type Balance = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const COLLECTION_ID: u64 = 0;

impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type WeightInfo = ();
}

pub struct Handler;

impl AuctionHandler<AccountId, Balance, BlockNumber, AuctionId> for Handler {
    fn on_new_bid(now: BlockNumber, id: AuctionId, new_bid: (AccountId, Balance), last_bid: Option<(AccountId, Balance)>) -> OnNewBidResult<BlockNumber> {
        //Test with Alice bid
        if new_bid.0 == ALICE {
            OnNewBidResult {
                accept_bid: true,
                auction_end_change: Change::NoChange,
            }
        } else {
            OnNewBidResult {
                accept_bid: false,
                auction_end_change: Change::NoChange,
            }
        }
    }

    fn on_auction_ended(_id: AuctionId, _winner: Option<(AccountId, Balance)>) {}
}


parameter_types! {
    pub const AuctionTimeToClose: u64 = 100; //Test auction end within 100 blocks
}

impl Config for Runtime {
    type Event = Event;
    type AuctionTimeToClose = AuctionTimeToClose;
    type AuctionId = AuctionId;
    type Handler = Handler;
    type Currency = Balances;
}

impl orml_nft::Config for Runtime {
    type ClassId = u32;
    type TokenId = u64;
    type ClassData = NFTModule::NftClassData<Balance>;
    type TokenData = NFTModule::NftAssetData<Balance>;
}

use frame_system::Call as SystemCall;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
        OrmlNft: orml_nft::{Module, Storage, Config<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        NftAuctionModule: auction::{Module, Call, Storage, Event<T>},
	}
);


pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: vec![(ALICE, 100000)],
        }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn last_event() -> Event {
    frame_system::Module::<Runtime>::events()
        .pop()
        .expect("Event expected")
        .event
}

