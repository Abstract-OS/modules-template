//! # Template API
//!
//! `template_namespace::template_api`

use abstract_sdk::os::api;
use abstract_sdk::os::objects::{AnsAsset, AssetEntry};
use cosmwasm_schema::QueryResponses;

pub type ProviderName = String;
pub type LpToken = AnsAsset;

/// The callback id for staking over ibc
pub const IBC_STAKING_PROVIDER_ID: u32 = 22335;

pub const TEMPLATE_API_CONTRACT_NAME: &str = "template_namespace:template_api";

pub type ExecuteMsg = api::ExecuteMsg<TemplateApiExecuteMsg>;
pub type QueryMsg = api::QueryMsg<TemplateApiQueryMsg>;

impl api::ApiExecuteMsg for TemplateApiExecuteMsg {}

impl api::ApiQueryMsg for TemplateApiQueryMsg {}

/// A request message that's sent to this staking api
#[cosmwasm_schema::cw_serde]
pub struct TemplateApiExecuteMsg {
    pub provider: ProviderName,
    pub action: CwStakingAction,
}

#[cosmwasm_schema::cw_serde]
/// Possible actions to perform on the staking contract
pub enum CwStakingAction {
    /// Stake a given LP token
    Stake { lp_token: LpToken },
    /// Unstake a given LP token
    Unstake { lp_token: LpToken },
    /// Claim rewards for a given LP token
    Claim { lp_token_name: AssetEntry },
}

#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
pub enum TemplateApiQueryMsg {}
