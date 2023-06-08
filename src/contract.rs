use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Config, Organization, SubscriptionPlan, CONFIG, ORGANIZATIONS, ORGANIZATION_ID,
    SUBSCRIPTION_ID, SUBSCRIPTION_PLANS, SUBSCRIPTION_PLAN_ID, USER_ORGANIZATIONS,
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

    // Initialize the ID counters
    ORGANIZATION_ID.save(deps.storage, &0)?;
    SUBSCRIPTION_PLAN_ID.save(deps.storage, &0)?;
    SUBSCRIPTION_ID.save(deps.storage, &0)?;

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
        } => execute_create_organization(deps, env, info, name, description, website, metadata),
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

fn execute_create_organization(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    description: String,
    website: Option<String>,
    metadata: Option<BTreeMap<String, String>>,
) -> Result<Response, ContractError> {
    // Load and save the ID counter
    let organization_id = ORGANIZATION_ID.load(deps.storage)? + 1;
    ORGANIZATION_ID.save(deps.storage, &organization_id)?;

    // Create the organization
    let organization = Organization {
        owner: info.clone().sender,
        name,
        description,
        website,
        metadata,
    };
    ORGANIZATIONS.save(deps.storage, organization_id, &organization)?;

    // Load the user's list of organizations
    let mut user_organizations = USER_ORGANIZATIONS
        .may_load(deps.storage, info.clone().sender)?
        .unwrap_or_default();

    // Add the organization to the user's list of organizations
    user_organizations.push(organization_id);
    USER_ORGANIZATIONS.save(deps.storage, info.sender, &user_organizations)?;

    Ok(Response::new()
        .add_attribute("action", "create_organization")
        .add_attribute("organization_id", organization_id.to_string()))
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
