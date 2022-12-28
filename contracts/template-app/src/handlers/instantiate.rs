use crate::contract::{TemplateApp, TemplateAppResult};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use template_namespace::template_app::state::{Config, CONFIG};
use template_namespace::template_app::{TemplateAppInstantiateMsg, TEMPLATE_APP_CONTRACT_NAME};

/// Initial instantiation of the contract
pub fn instantiate_handler(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: TemplateApp,
    _msg: TemplateAppInstantiateMsg,
) -> TemplateAppResult {
    CONFIG.save(_deps.storage, &Config {})?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", TEMPLATE_APP_CONTRACT_NAME))
}
