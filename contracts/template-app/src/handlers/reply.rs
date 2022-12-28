use crate::contract::{TemplateApp, TemplateAppResult};
use cosmwasm_std::{DepsMut, Env, Reply, Response};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _app: TemplateApp,
    _reply: Reply,
) -> TemplateAppResult {
    Ok(Response::default())
}
