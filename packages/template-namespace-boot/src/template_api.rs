use boot_core::prelude::boot_contract;

use boot_core::{BootEnvironment, Contract, IndexResponse, TxResponse};

use abstract_os::api;
use cosmwasm_std::Empty;
use template_namespace::template_api::{TemplateApiExecuteMsg, TemplateApiQueryMsg};

type ApiExecuteMsg = api::ExecuteMsg<TemplateApiExecuteMsg>;
type ApiQueryMsg = api::QueryMsg<TemplateApiQueryMsg>;

/// Contract wrapper for interacting with BOOT
#[boot_contract(Empty, ApiExecuteMsg, ApiQueryMsg, Empty)]
pub struct TemplateApi<Chain>;

/// implement chain-generic functions
impl<Chain: BootEnvironment> TemplateApi<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(id: &str, chain: &Chain) -> Self {
        Self(Contract::new(id, chain).with_wasm_path("cw_staking"))
    }
}
