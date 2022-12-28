use crate::contract::TemplateApp;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};
use template_namespace::template_app::state::CONFIG;
use template_namespace::template_app::{ConfigResponse, TemplateAppQueryMsg};

const _DEFAULT_PAGE_SIZE: u8 = 5;
const _MAX_PAGE_SIZE: u8 = 20;

/// Handle queries sent to this app.
pub fn query_handler(
    _deps: Deps,
    _env: Env,
    _app: &TemplateApp,
    msg: TemplateAppQueryMsg,
) -> StdResult<Binary> {
    match msg {
        TemplateAppQueryMsg::Config {} => to_binary(&query_config(_deps)?),
    }
}

/// Returns the current configuration of the contract.
fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {})
}
