use abstract_app::export_endpoints;
use abstract_app::AppContract;

use cosmwasm_std::Response;

use template_namespace::template_app::{
    TemplateAppExecuteMsg, TemplateAppInstantiateMsg, TemplateAppMigrateMsg, TemplateAppQueryMsg,
    TEMPLATE_APP_CONTRACT_NAME,
};

use crate::dependencies::TEMPLATE_DEPS;

use crate::error::TemplateAppError;
use crate::handlers::{self};

// As an app writer, the only changes necessary to this file are with the handlers and API dependencies on the `APP` const.
pub type TemplateApp = AppContract<
    TemplateAppError,
    TemplateAppExecuteMsg,
    TemplateAppInstantiateMsg,
    TemplateAppQueryMsg,
    TemplateAppMigrateMsg,
>;

pub type TemplateAppResult = Result<Response, TemplateAppError>;

/// The initial version of the app, which will use the package version if not altered
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Expected replies
pub const EXAMPLE_REPLY_ID: u64 = 69420;
pub const INSTANTIATE_REPLY_ID: u64 = 0u64;
pub const LP_PROVISION_REPLY_ID: u64 = 1u64;

/// Used as the foundation for building your app.
/// All entrypoints are executed through this const (`instantiate`, `query`, `execute`, `migrate`)
const APP: TemplateApp = TemplateApp::new(TEMPLATE_APP_CONTRACT_NAME, MODULE_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_query(handlers::query_handler)
    .with_execute(handlers::execute_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(EXAMPLE_REPLY_ID, handlers::example_reply_handler)])
    .with_dependencies(TEMPLATE_DEPS);

// Export the endpoints for this contract
export_endpoints!(APP, TemplateApp);
