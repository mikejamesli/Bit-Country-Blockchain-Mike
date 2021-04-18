#![cfg_attr(not(feature = "std"), no_std)]
// Disable the following two lints since they originate from an external macro (namely decl_storage)
#![allow(clippy::string_lit_as_bytes)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    ensure,
    traits::{Currency,ExistenceRequirement, Get},Parameter, 
    weights::Weight,
    StorageMap, StorageValue,
};

use codec::{Decode, Encode};
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One}, RuntimeDebug,
    DispatchError, DispatchResult,
};
use sp_std::vec::Vec;

use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use primitives::{CountryId};

pub struct PoolLogicHandler;

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, Clone, RuntimeDebug)]
pub struct Pool<BlockNumber> {
    name: Vec<u8>,
    start: BlockNumber,
    end: BlockNumber,
    reward_percentage: u32,
}

pub trait Config:
frame_system::Config
+ pallet_balances::Config
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// The Pool ID type
    type PoolId: Parameter
    + Member
    + AtLeast32BitUnsigned
    + Default
    + Copy
    + MaybeSerializeDeserialize
    + Bounded;

    /// result
    type Currency: Currency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Config> as Staking {

        pub CountryStakingPools get(fn country_staking_pools):  map hasher(twox_64_concat) CountryId => Vec<T::PoolId>;

        pub Pools get(fn pools): map hasher(twox_64_concat) T::PoolId => Option<Pool<T::BlockNumber>>;

        pub AccountStakingPools get(fn account_staking_pools): double_map hasher(twox_64_concat) T::PoolId, hasher(twox_64_concat) T::AccountId => T::Balance;

        pub StakingPoolAccountList get(fn staking_pool_acount_list): map hasher(twox_64_concat) T::PoolId => Vec<T::PoolId>;

        pub PoolsEnded get(fn pools_ended): map hasher(twox_64_concat) T::BlockNumber => Vec<T::PoolId>;

        pub StakingRewards get(fn staking_rewards): double_map hasher(twox_64_concat) T::PoolId, hasher(twox_64_concat) T::AccountId => T::Balance;

        pub StakingPoolCount get(fn staking_pool_count): T::PoolId;

        pub PoolsTotalStaked get(fn pools_total_staked): u64;
    }
}
decl_event!(
    pub enum Event<T> where
        <T as frame_system::Config>::AccountId,
        <T as pallet_balances::Config>::Balance, 
        <T as Config>::PoolId,
    {
        PoolCreated(AccountId, PoolId),
        Stake(PoolId, AccountId, Balance),
        Claim(PoolId, AccountId, Balance),
        Unstake(PoolId),
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000]
        fn create_staking_pool(origin,  country_id: CountryId, name: Vec<u8>, start: T::BlockNumber, end: T::BlockNumber) {
            let from = ensure_signed(origin)?;
            
            let pool_id = Self::new_staking_pool(from.clone(), name, start, end, 10)?;

            if CountryStakingPools::<T>::contains_key(&country_id){
                CountryStakingPools::<T>::try_mutate(
                    &country_id, |pools| -> DispatchResult {
                        pools.push(pool_id);
                        Ok(())
                })?;
            } else {
                let mut new_pool = Vec::<T::PoolId>::new();
                new_pool.push(pool_id);
                CountryStakingPools::<T>::insert(country_id, new_pool)
            }

            if PoolsEnded::<T>::contains_key(&end){
                PoolsEnded::<T>::try_mutate(
                    &end, |pools| -> DispatchResult {
                        pools.push(pool_id);
                        Ok(())
                })?;
            } else {
                let mut new_pool = Vec::<T::PoolId>::new();
                new_pool.push(pool_id);
                PoolsEnded::<T>::insert(end, new_pool)
            }

            Self::deposit_event(RawEvent::PoolCreated(from, pool_id));
        }

        #[weight = 10_000]
        fn stake(origin, pool_id: T::PoolId, value: T::Balance) {
            let from = ensure_signed(origin)?;
        }

        #[weight = 10_000]
        fn unstake(origin, pool_id: T::PoolId, value: T::Balance) {
            let from = ensure_signed(origin)?;
        }

        #[weight = 10_000]
        fn claim(origin, pool_id: T::PoolId) {
            let from = ensure_signed(origin)?;
        }

        fn on_finalize(now: T::BlockNumber) {
        }
    }
}

decl_error! {
    /// Error for auction module.
    pub enum Error for Module<T: Config> {
        NoAvailablePoolId,
    }
}

impl<T: Config> Module<T> {
    fn new_staking_pool(
        _recipient: T::AccountId,
        _name: Vec<u8>,
        _start: T::BlockNumber,
        _end: T::BlockNumber,
        _reward_percentage: u32,
    ) -> Result<T::PoolId, DispatchError> {
        let pool: Pool<T::BlockNumber> = Pool {
           name: _name,
           start: _start,
           end: _end,
           reward_percentage: _reward_percentage,
        };

        let pool_id: T::PoolId =
        <StakingPoolCount<T>>::try_mutate(|n| -> Result<T::PoolId, DispatchError> {
            let id = *n;
            ensure!(
                id != T::PoolId::max_value(),
                Error::<T>::NoAvailablePoolId
            );
            *n += One::one();
            Ok(id)
        })?;

        <Pools<T>>::insert(pool_id, pool);

        Ok(pool_id)
    }
}

