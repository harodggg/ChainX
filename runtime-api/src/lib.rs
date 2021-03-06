// Copyright 2018-2019 Chainpool.

#![allow(clippy::too_many_arguments)]
#![cfg_attr(not(feature = "std"), no_std)]

use rstd::collections::btree_map::BTreeMap;
use rstd::prelude::Vec;
use sr_primitives::traits::AuthorityIdFor;

use client::decl_runtime_apis;

use chainx_primitives::{AccountIdForApi, Balance, BlockNumber, Timestamp};

pub mod xassets_api {
    use super::*;
    use rstd::collections::btree_map::BTreeMap;
    use xassets::{Asset, AssetType, Memo, Token};
    use xprocess::WithdrawalLimit;
    use xr_primitives::AddrStr;

    decl_runtime_apis! {
        pub trait XAssetsApi {
            fn valid_assets() -> Vec<Token>;
            fn all_assets() -> Vec<(Asset, bool)>;
            fn valid_assets_of(who: AccountIdForApi) -> Vec<(Token, BTreeMap<AssetType, Balance>)>;
            fn withdrawal_list_of(chain: xassets::Chain) -> Vec<xrecords::RecordInfo<AccountIdForApi, Balance, BlockNumber, Timestamp>>;
            fn deposit_list_of(chain: xassets::Chain) -> Vec<xrecords::RecordInfo<AccountIdForApi, Balance, BlockNumber, Timestamp>>;
            fn verify_address(token: Token, addr: AddrStr, ext: Memo) -> Result<(), Vec<u8>>;
            fn withdrawal_limit(token: Token) -> Option<WithdrawalLimit<Balance>>;
        }
    }
}

pub mod xmining_api {
    use super::*;
    use xassets::Token;

    decl_runtime_apis! {
        pub trait XMiningApi {
            fn jackpot_accountid_for_unsafe(who: AccountIdForApi) -> AccountIdForApi;
            fn multi_jackpot_accountid_for_unsafe(who: Vec<AccountIdForApi>) -> Vec<AccountIdForApi>;
            fn token_jackpot_accountid_for_unsafe(token: Token) -> AccountIdForApi;
            fn multi_token_jackpot_accountid_for_unsafe(token: Vec<Token>) -> Vec<AccountIdForApi>;
            fn asset_power(token: Token) -> Option<Balance>;
        }
    }
}

pub mod xspot_api {
    use super::*;
    use xassets::Token;

    decl_runtime_apis! {
        pub trait XSpotApi {
            fn aver_asset_price(token: Token) -> Option<Balance>;
        }
    }
}

pub mod xfee_api {
    use super::*;

    decl_runtime_apis! {
        pub trait XFeeApi {
            fn transaction_fee(call: Vec<u8>, encoded_len: u64) -> Option<u64>;

            fn fee_weight_map() -> BTreeMap<Vec<u8>, u64>;
        }
    }
}

pub mod xsession_api {
    use super::*;

    decl_runtime_apis! {
        pub trait XSessionApi {
            fn pubkeys_for_validator_name(name: Vec<u8>) -> Option<(AccountIdForApi, Option<AuthorityIdFor<Block>>)>;
        }
    }
}

pub mod xstaking_api {
    use super::*;

    decl_runtime_apis! {
        pub trait XStakingApi {
            fn intention_set() -> Vec<AccountIdForApi>;
            // T::SessionKey should use AuthorityId here and ChainX is able to compile, but the tool depdendent on ChainX fails to compile when using AuthorityId.
            // 2019-05-25: Compile ERROR: the return type of a function must have a statically known size
            // 2019-07-15 ChainX is able compile, but the tool depdendent on ChainX is unable to compile.
            fn intentions_info_common() -> Vec<xstaking::IntentionInfoCommon<AccountIdForApi, Balance, AccountIdForApi, BlockNumber>>;
            fn intention_info_common_of(intention: &AccountIdForApi) -> Option<xstaking::IntentionInfoCommon<AccountIdForApi, Balance, AccountIdForApi, BlockNumber>>;
        }
    }
}

pub mod xbridge_api {
    use super::*;
    use xassets::Chain;
    use xbridge_common::types::{GenericAllSessionInfo, GenericTrusteeIntentionProps};
    decl_runtime_apis! {
        pub trait XBridgeApi {
            /// generate a mock trustee info
            fn mock_new_trustees(chain: Chain, candidates: Vec<AccountIdForApi>) -> Result<GenericAllSessionInfo<AccountIdForApi>, Vec<u8>>;

            fn trustee_props_for(who: AccountIdForApi) ->  BTreeMap<xassets::Chain, GenericTrusteeIntentionProps>;

            fn trustee_session_info() -> BTreeMap<xassets::Chain, GenericAllSessionInfo<AccountIdForApi>>;

            fn trustee_session_info_for(chain: Chain, number: Option<u32>) -> Option<(u32, GenericAllSessionInfo<AccountIdForApi>)>;
        }
    }
}

pub mod xcontracts_api {
    use super::*;
    use xassets::Token;
    use xr_primitives::{ContractExecResult, GetStorageResult, XRC20Selector};

    decl_runtime_apis! {
        /// The API to interact with contracts without using executive.
        pub trait XContractsApi {
            /// Perform a call from a specified account to a given contract.
            ///
            /// See the contracts' `call` dispatchable function for more details.
            fn call(
                origin: AccountIdForApi,
                dest: AccountIdForApi,
                value: Balance,
                gas_limit: u64,
                issue_gas: bool,
                input_data: Vec<u8>,
            ) -> (ContractExecResult, Balance);

            /// Query a given storage key in a given contract.
            ///
            /// Returns `Ok(Some(Vec<u8>))` if the storage value exists under the given key in the
            /// specified account and `Ok(None)` if it doesn't. If the account specified by the address
            /// doesn't exist, or doesn't have a contract or if the contract is a tombstone, then `Err`
            /// is returned.
            fn get_storage(address: AccountIdForApi, key: [u8; 32]) -> GetStorageResult;

            fn xrc20_call(
                token: Token,
                selector: XRC20Selector,
                data: Vec<u8>,
            ) -> ContractExecResult;
        }
    }
}
