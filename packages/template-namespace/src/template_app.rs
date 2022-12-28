//! # App Template
//!
//! `template_namespace::template_app` is an app which allows users to ...
//!
//! ## Creation
//! The contract can be added on an OS by calling [`ExecuteMsg::InstallModule`](crate::manager::ExecuteMsg::InstallModule) on the manager of the os.
//! ```rust
//!
//! use abstract_sdk::{
//!     os::{
//!         app,
//!         manager::ExecuteMsg,
//!         objects::module::ModuleInfo
//!     }
//! };
//! use cosmwasm_std::to_binary;
//! use template_namespace::template_app::TEMPLATE_APP_CONTRACT_NAME;
//!
//! let init_msg: app::InstantiateMsg<_> = todo!();
//!
//! let create_module_msg = ExecuteMsg::InstallModule {
//!     module: ModuleInfo::from_id_latest(TEMPLATE_APP_CONTRACT_NAME).unwrap(),
//!     init_msg: Some(to_binary(&init_msg).unwrap()),
//! };
//! // Call create_module_msg on manager
//! ```
//!
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` on [`crate::manager`] with `crate::APP` as module.

use abstract_sdk::os::app;
use cosmwasm_schema::QueryResponses;

pub const TEMPLATE_APP_CONTRACT_NAME: &str = "template_namespace:template_app";

pub mod state {
    use cosmwasm_schema::cw_serde;
    use cw_storage_plus::Item;

    /// General configuration of the app
    #[cw_serde]
    pub struct Config {}

    pub const CONFIG: Item<Config> = Item::new("config");
}

/// Impls for being able to call methods on the template_app directly
impl app::AppExecuteMsg for TemplateAppExecuteMsg {}
impl app::AppQueryMsg for TemplateAppQueryMsg {}

/// Migrate msg
#[cosmwasm_schema::cw_serde]
pub struct TemplateAppMigrateMsg {}

/// Init msg
#[cosmwasm_schema::cw_serde]
pub struct TemplateAppInstantiateMsg {}

#[cosmwasm_schema::cw_serde]
pub enum TemplateAppExecuteMsg {
    UpdateConfig {},
}

#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
pub enum TemplateAppQueryMsg {
    /// Returns the current config
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}
