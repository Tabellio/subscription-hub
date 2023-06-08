#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Organization, CONFIG, ORGANIZATIONS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-subscription-hub";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { admin: info.sender };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", config.admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateOrganization {
            name,
            description,
            website,
            metadata,
        } => {
            unimplemented!()
        }
        ExecuteMsg::CreateSubscriptionPlan {
            name,
            description,
            payment_amount,
            expiration,
            features,
            metadata,
            cancelable,
            refundable,
        } => {
            unimplemented!()
        }
        ExecuteMsg::SubscribePlan { plan_id } => {
            unimplemented!()
        }
        ExecuteMsg::CancelPlan { plan_id } => {
            unimplemented!()
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Organization { organization_id } => {
            to_binary(&query_organization(deps, organization_id)?)
        }
        QueryMsg::UserOrganizations { user_address } => {
            unimplemented!()
        }
        QueryMsg::SubscriptionPlan { plan_id } => {
            unimplemented!()
        }
        QueryMsg::SubscriptionPlans { organization_id } => {
            unimplemented!()
        }
        QueryMsg::Subscription { subscription_id } => {
            unimplemented!()
        }
        QueryMsg::UserSubscriptions { user_address } => {
            unimplemented!()
        }
        QueryMsg::SubscriptionPlanSubscriptions { plan_id } => {
            unimplemented!()
        }
    }
}

fn query_organization(deps: Deps, organization_id: u32) -> StdResult<Organization> {
    let organization = ORGANIZATIONS.load(deps.storage, organization_id)?;
    Ok(organization)
}
