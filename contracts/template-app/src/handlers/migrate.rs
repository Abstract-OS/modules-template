use cosmwasm_std::{DepsMut, Env, Response};
use template_namespace::template_app::TemplateAppMigrateMsg;

use crate::contract::{TemplateApp, TemplateAppResult};

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _app: TemplateApp,
    _msg: TemplateAppMigrateMsg,
) -> TemplateAppResult {
    Ok(Response::default())
}
