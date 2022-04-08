use crate::utils::{OrderBy, PollStatus};
use cosmwasm_std::{Binary, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/**
{
    "cw20_token": "terra1lzfrsy38l34uzrlma3fm3hsktv848pcgdnlcj7",
    "quorum": "0.1",
    "threshold": "0.5",
    "voting_period": 50,
    "timelock_period": 50,
    "proposal_deposit": "10",
    "snapshot_period": 30
}
**/
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_token: String,
    pub quorum: Decimal,
    pub threshold: Decimal,
    pub voting_period: u64,
    pub timelock_period: u64,
    pub proposal_deposit: Uint128,
    pub snapshot_period: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Unstake {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    Balance {},
    Staker {
        address: String,
    },
    Poll {
        poll_id: u64,
    },
    /*
    {"polls": {
        "filter": "in_progress"
        }
    }
    */
    Polls {
        filter: Option<PollStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
    Voters {
        poll_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    UnbondBluna {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BlunaExecuteMsg {
    Unbond {},
    WithdrawUnbonded {},
}