use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Config, DurationUnit, Organization, Subscription, SubscriptionPlan, CONFIG, ORGANIZATIONS,
    ORGANIZATION_ID, ORGANIZATION_SUBSCRIPTION_PLANS, SUBSCRIPTIONS, SUBSCRIPTION_ID,
    SUBSCRIPTION_PLANS, SUBSCRIPTION_PLAN_ID, SUBSCRIPTION_PLAN_SUBSCRIPTIONS, USER_ORGANIZATIONS,
    USER_SUBSCRIPTIONS,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:subscription-hub";
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
            organization_id,
            name,
            description,
            price,
            duration,
            duration_unit,
            features,
            metadata,
            cancelable,
            refundable,
        } => execute_create_subscription_plan(
            deps,
            env,
            info,
            organization_id,
            name,
            description,
            price,
            duration,
            duration_unit,
            features,
            metadata,
            cancelable,
            refundable,
        ),
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

fn execute_create_subscription_plan(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    organization_id: u32,
    name: String,
    description: String,
    price: Uint128,
    duration: u8,
    duration_unit: DurationUnit,
    features: Option<Vec<String>>,
    metadata: Option<BTreeMap<String, String>>,
    cancelable: bool,
    refundable: bool,
) -> Result<Response, ContractError> {
    // Load the organization
    let organization = ORGANIZATIONS.load(deps.storage, organization_id)?;

    // Check that the sender is the organization owner
    if info.sender != organization.owner {
        return Err(ContractError::Unauthorized {});
    };

    // Load and save the ID counter
    let subscription_plan_id = SUBSCRIPTION_PLAN_ID.load(deps.storage)? + 1;
    SUBSCRIPTION_PLAN_ID.save(deps.storage, &subscription_plan_id)?;

    // Create the subscription plan
    let subscription_plan = SubscriptionPlan {
        organization_id,
        name,
        description,
        price,
        duration,
        duration_unit,
        features,
        metadata,
        cancelable,
        refundable,
    };
    SUBSCRIPTION_PLANS.save(deps.storage, subscription_plan_id, &subscription_plan)?;

    // Load the organization's list of subscription plans
    let mut organization_subscription_plans = ORGANIZATION_SUBSCRIPTION_PLANS
        .may_load(deps.storage, organization_id)?
        .unwrap_or_default();

    // Add the subscription plan to the organization's list of subscription plans
    organization_subscription_plans.push(subscription_plan_id);
    ORGANIZATION_SUBSCRIPTION_PLANS.save(
        deps.storage,
        organization_id,
        &organization_subscription_plans,
    )?;

    Ok(Response::new()
        .add_attribute("action", "create_subscription_plan")
        .add_attribute("subscription_plan_id", subscription_plan_id.to_string()))
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
        QueryMsg::OrganizationSubscriptionPlans { organization_id } => to_binary(
            &query_organization_subscription_plans(deps, organization_id)?,
        ),
        QueryMsg::Subscription { subscription_id } => {
            to_binary(&query_subscription(deps, subscription_id)?)
        }
        QueryMsg::UserSubscriptions { user_address } => {
            to_binary(&query_user_subscriptions(deps, user_address)?)
        }
        QueryMsg::SubscriptionPlanSubscriptions { plan_id } => {
            to_binary(&query_subscription_plan_subscriptions(deps, plan_id)?)
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

fn query_organization_subscription_plans(
    deps: Deps,
    organization_id: u32,
) -> StdResult<Vec<SubscriptionPlan>> {
    // Load organization subscription plans
    let subscription_plan_ids =
        ORGANIZATION_SUBSCRIPTION_PLANS.load(deps.storage, organization_id)?;

    // Load subscription plans for each subscription plan id
    let subscription_plans = subscription_plan_ids
        .iter()
        .map(|id| SUBSCRIPTION_PLANS.load(deps.storage, *id))
        .collect::<StdResult<Vec<SubscriptionPlan>>>()?;

    Ok(subscription_plans)
}

fn query_subscription(deps: Deps, subscription_id: u64) -> StdResult<Subscription> {
    let subscription = SUBSCRIPTIONS.load(deps.storage, subscription_id)?;

    Ok(subscription)
}

fn query_user_subscriptions(deps: Deps, user_address: String) -> StdResult<Vec<Subscription>> {
    // Validate user address
    let user_addr = deps.api.addr_validate(&user_address)?;

    // Load user subscriptions
    let subscription_ids = USER_SUBSCRIPTIONS.load(deps.storage, user_addr)?;

    // Load subscriptions for each subscription id
    let subscriptions = subscription_ids
        .iter()
        .map(|id| SUBSCRIPTIONS.load(deps.storage, *id))
        .collect::<StdResult<Vec<Subscription>>>()?;

    Ok(subscriptions)
}

fn query_subscription_plan_subscriptions(deps: Deps, plan_id: u64) -> StdResult<Vec<Subscription>> {
    // Load subscription plan subscriptions
    let subscription_ids = SUBSCRIPTION_PLAN_SUBSCRIPTIONS.load(deps.storage, plan_id)?;

    // Load subscriptions for each subscription id
    let subscriptions = subscription_ids
        .iter()
        .map(|id| SUBSCRIPTIONS.load(deps.storage, *id))
        .collect::<StdResult<Vec<Subscription>>>()?;

    Ok(subscriptions)
}
