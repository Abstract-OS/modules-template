use abstract_os::app;
use abstract_os::app::{BaseExecuteMsg, BaseQueryMsg};
use boot_core::prelude::{boot_contract, BootExecute, BootQuery};
use boot_core::{BootEnvironment, BootError, Contract, IndexResponse, TxResponse};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use serde::Serialize;

use template_namespace::template_app::{
    TemplateAppExecuteMsg, TemplateAppInstantiateMsg, TemplateAppMigrateMsg, TemplateAppQueryMsg,
};

type AppInstantiateMsg = app::InstantiateMsg<TemplateAppInstantiateMsg>;
type AppExecuteMsg = app::ExecuteMsg<TemplateAppExecuteMsg>;
type AppQueryMsg = app::QueryMsg<TemplateAppQueryMsg>;
type AppMigrateMsg = app::MigrateMsg<TemplateAppMigrateMsg>;

/// Contract wrapper for deploying with BOOT
#[boot_contract(AppInstantiateMsg, AppExecuteMsg, AppQueryMsg, AppMigrateMsg)]
pub struct TemplateApp<Chain>;

impl<Chain: BootEnvironment> TemplateApp<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(Contract::new(name, chain).with_wasm_path("template_app"))
    }

    /// Temporary helper to query the app explicitly
    pub fn query_app<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: TemplateAppQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the app base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: BaseQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the app explicitly
    pub fn execute_app(
        &self,
        execute_msg: TemplateAppExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the app base explicitly
    pub fn execute_base(
        &self,
        execute_msg: BaseExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::Base(execute_msg), coins)
    }
}
