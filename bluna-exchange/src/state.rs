use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, CanonicalAddr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use cw_storage_plus::Map;

use crate::utils::{OrderBy, PollStatus, VoteInfo};
use std::cmp::Ordering;

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";
static KEY_TMP_POLL_ID: &[u8] = b"tmp_poll_id";

static PREFIX_POLL_INDEXER: &[u8] = b"poll_indexer";
static PREFIX_POLL_VOTER: &[u8] = b"poll_voter";
static PREFIX_POLL: &[u8] = b"poll";
static PREFIX_BANK: &[u8] = b"bank";

/// default information & parameters for the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,        // owner of the contract
    pub bluna_token: CanonicalAddr,  // cw20 token contract
    pub cw1155_token: CanonicalAddr, // cw1155 token contract
    pub unbond_contract: CanonicalAddr,
}

// state for the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_addr: CanonicalAddr, // address of this contract
    pub unbonded_amount: Uint128,
    pub withdrawn_amount: Uint128,
}

pub const TOKENS: Map<&str, Uint128> = Map::new("tokens");

// token manager maps to each client
#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenManager {
    pub share: Uint128,                       // total staked balance
    pub locked_balance: Vec<(u64, VoteInfo)>, // maps poll_id to weight voted
}

pub fn config_store(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, KEY_CONFIG)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, KEY_CONFIG)
}

pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, KEY_STATE)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, KEY_STATE)
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn bank_store(storage: &mut dyn Storage) -> Bucket<TokenManager> {
    bucket(storage, PREFIX_BANK)
}

pub fn bank_read(storage: &dyn Storage) -> ReadonlyBucket<TokenManager> {
    bucket_read(storage, PREFIX_BANK)
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start(start_after: Option<u64>) -> Option<Vec<u8>> {
    start_after.map(|id| {
        let mut v = id.to_be_bytes().to_vec();
        v.push(1);
        v
    })
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_end(start_after: Option<u64>) -> Option<Vec<u8>> {
    start_after.map(|id| id.to_be_bytes().to_vec())
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|addr| {
        let mut v = addr.as_slice().to_vec();
        v.push(1);
        v
    })
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_end_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|addr| addr.as_slice().to_vec())
}

pub fn read_poll_voters<'a>(
    storage: &'a dyn Storage,
    poll_id: u64,
    start_after: Option<CanonicalAddr>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<Vec<(CanonicalAddr, VoteInfo)>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let (start, end, order_by) = match order_by {
        Some(OrderBy::Asc) => (calc_range_start_addr(start_after), None, OrderBy::Asc),
        _ => (None, calc_range_end_addr(start_after), OrderBy::Desc),
    };

    let voters: ReadonlyBucket<'a, VoteInfo> =
        ReadonlyBucket::multilevel(storage, &[PREFIX_POLL_VOTER, &poll_id.to_be_bytes()]);
    voters
        .range(start.as_deref(), end.as_deref(), order_by.into())
        .take(limit)
        .map(|item| {
            let (k, v) = item?;
            Ok((CanonicalAddr::from(k), v))
        })
        .collect()
}
