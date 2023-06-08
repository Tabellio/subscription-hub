#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Config, Organization, SubscriptionPlan, CONFIG, ORGANIZATIONS, SUBSCRIPTION_PLANS,
    USER_ORGANIZATIONS,
};

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
            to_binary(&query_user_organizations(deps, user_address)?)
        }
        QueryMsg::SubscriptionPlan { plan_id } => {
            to_binary(&query_subscription_plan(deps, plan_id)?)
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

fn query_user_organizations(deps: Deps, user_address: String) -> StdResult<Vec<Organization>> {
    // Validate user address
    let user_addr = deps.api.addr_validate(&user_address)?;

    // Load user organizations
    let organization_ids = USER_ORGANIZATIONS.load(deps.storage, user_addr)?;

    // Load organizations for each organization id
    let organizations = organization_ids
        .iter()
        .map(|id| ORGANIZATIONS.load(deps.storage, *id))
        .collect::<StdResult<Vec<Organization>>>()?;

    Ok(organizations)
}

fn query_subscription_plan(deps: Deps, plan_id: u64) -> StdResult<SubscriptionPlan> {
    let subscription_plan = SUBSCRIPTION_PLANS.load(deps.storage, plan_id)?;

    Ok(subscription_plan)
}
