use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use template_namespace::template_app::TemplateAppExecuteMsg;

use crate::contract::{TemplateApp, TemplateAppResult};

/// Handle the [`TemplateAppExecuteMsg`]s sent to this app.
pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: TemplateApp,
    msg: TemplateAppExecuteMsg,
) -> TemplateAppResult {
    match msg {
        TemplateAppExecuteMsg::UpdateConfig { .. } => update_config(app, (deps, env, info)),
    }
}

fn update_config(_app: TemplateApp, _ctx: (DepsMut, Env, MessageInfo)) -> TemplateAppResult {
    Ok(Response::default())
}
