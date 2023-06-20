use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, OrganizationResponse, QueryMsg, SubscriptionPlanResponse,
    SubscriptionResponse,
};
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
        ExecuteMsg::SubscribePlan { plan_id } => execute_subscribe_plan(deps, env, info, plan_id),
        ExecuteMsg::CancelPlan { plan_id } => execute_cancel_plan(deps, env, info, plan_id),
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

fn execute_subscribe_plan(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    plan_id: u64,
) -> Result<Response, ContractError> {
    // Load the subscription plan
    let subscription_plan = SUBSCRIPTION_PLANS.load(deps.storage, plan_id)?;

    // Calculate the expiration date based on the duration and duration unit
    let time_unit = match subscription_plan.duration_unit {
        DurationUnit::Day => 86400,
        DurationUnit::Week => 604800,
        DurationUnit::Month => 2592000,
        DurationUnit::Year => 31536000,
    };
    let expiration = env
        .block
        .time
        .plus_seconds(subscription_plan.duration as u64 * time_unit);

    // Load and save the subscription id counter
    let subscription_id = SUBSCRIPTION_ID.load(deps.storage)? + 1;
    SUBSCRIPTION_ID.save(deps.storage, &subscription_id)?;

    // Create the subscription
    let subscription = Subscription {
        subscriber: info.clone().sender,
        plan_id,
        expiration,
        canceled: false,
    };

    // Save the subscription
    SUBSCRIPTIONS.save(deps.storage, subscription_id, &subscription)?;

    // Update the user's list of subscriptions
    match USER_SUBSCRIPTIONS.may_load(deps.storage, (info.clone().sender, plan_id))? {
        // Update an existing subscription
        Some(existing_subscription_id) => {
            let existing_subscription =
                SUBSCRIPTIONS.load(deps.storage, existing_subscription_id)?;

            // Check if the subscription is still active
            if !existing_subscription.canceled || existing_subscription.expiration > env.block.time
            {
                return Err(ContractError::AlreadySubscribed {});
            }

            // Update the existing subscription
            USER_SUBSCRIPTIONS.save(
                deps.storage,
                (info.clone().sender, plan_id),
                &subscription_id,
            )?
        }
        // Create a new subscription
        None => USER_SUBSCRIPTIONS.save(
            deps.storage,
            (info.clone().sender, plan_id),
            &subscription_id,
        )?,
    }

    // Update the subscription plan's list of subscriptions
    SUBSCRIPTION_PLAN_SUBSCRIPTIONS.save(deps.storage, (plan_id, info.sender), &subscription_id)?;

    Ok(Response::new()
        .add_attribute("action", "subscribe_plan")
        .add_attribute(
            "organization_id",
            subscription_plan.organization_id.to_string(),
        )
        .add_attribute("subscription_plan_id", plan_id.to_string())
        .add_attribute("subscription_id", subscription_id.to_string()))
}

fn execute_cancel_plan(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subscription_id: u64,
) -> Result<Response, ContractError> {
    // Load the subscription
    let mut subscription = SUBSCRIPTIONS.load(deps.storage, subscription_id)?;

    // Load the subscription plan
    let subscription_plan = SUBSCRIPTION_PLANS.load(deps.storage, subscription.plan_id)?;

    // Check that the sender is the subscription owner
    if info.sender != subscription.subscriber {
        return Err(ContractError::Unauthorized {});
    };

    // Check that the subscription is cancelable
    if !subscription_plan.cancelable {
        return Err(ContractError::NotCancelable {});
    };

    // Check that the subscription is not already canceled
    if subscription.canceled {
        return Err(ContractError::AlreadyCanceled {});
    };

    // Check that the subscription is not expired
    if env.block.time > subscription.expiration {
        return Err(ContractError::AlreadyExpired {});
    };

    // Cancel the subscription and save it
    subscription.canceled = true;
    SUBSCRIPTIONS.save(deps.storage, subscription_id, &subscription)?;

    // Remove the subscription from the user's list of subscriptions
    SUBSCRIPTION_PLAN_SUBSCRIPTIONS.remove(deps.storage, (subscription.plan_id, info.sender));

    Ok(Response::new()
        .add_attribute("action", "cancel_plan")
        .add_attribute("subscription_id", subscription_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
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
        QueryMsg::UserSubscriptions {
            user_address,
            start_after,
            limit,
        } => to_binary(&query_user_subscriptions(
            deps,
            user_address,
            start_after,
            limit,
        )?),
        QueryMsg::SubscriptionPlanSubscriptions {
            plan_id,
            start_after,
            limit,
        } => to_binary(&query_subscription_plan_subscriptions(
            deps,
            plan_id,
            start_after,
            limit,
        )?),
        QueryMsg::IsSubscribed {
            user_address,
            plan_id,
        } => to_binary(&query_is_subscribed(deps, env, user_address, plan_id)?),
    }
}

fn query_organization(deps: Deps, organization_id: u32) -> StdResult<OrganizationResponse> {
    let organization = ORGANIZATIONS.load(deps.storage, organization_id)?;

    Ok(OrganizationResponse {
        id: organization_id,
        data: organization,
    })
}

fn query_user_organizations(
    deps: Deps,
    user_address: String,
) -> StdResult<Vec<OrganizationResponse>> {
    // Validate user address
    let user_addr = deps.api.addr_validate(&user_address)?;

    // Load user organizations
    let organization_ids = USER_ORGANIZATIONS.load(deps.storage, user_addr)?;

    // Load organizations for each organization id
    let organizations = organization_ids
        .iter()
        .map(|id| {
            let organization = ORGANIZATIONS.load(deps.storage, *id)?;
            Ok(OrganizationResponse {
                id: *id,
                data: organization,
            })
        })
        .collect::<StdResult<Vec<OrganizationResponse>>>()?;

    Ok(organizations)
}

fn query_subscription_plan(deps: Deps, plan_id: u64) -> StdResult<SubscriptionPlanResponse> {
    let subscription_plan = SUBSCRIPTION_PLANS.load(deps.storage, plan_id)?;

    Ok(SubscriptionPlanResponse {
        id: plan_id,
        data: subscription_plan,
    })
}

fn query_organization_subscription_plans(
    deps: Deps,
    organization_id: u32,
) -> StdResult<Vec<SubscriptionPlanResponse>> {
    // Load organization subscription plans
    let subscription_plan_ids =
        ORGANIZATION_SUBSCRIPTION_PLANS.load(deps.storage, organization_id)?;

    // Load subscription plans for each subscription plan id
    let subscription_plans = subscription_plan_ids
        .iter()
        .map(|id| {
            let subscription_plan = SUBSCRIPTION_PLANS.load(deps.storage, *id)?;
            Ok(SubscriptionPlanResponse {
                id: *id,
                data: subscription_plan,
            })
        })
        .collect::<StdResult<Vec<SubscriptionPlanResponse>>>()?;

    Ok(subscription_plans)
}

fn query_subscription(deps: Deps, subscription_id: u64) -> StdResult<SubscriptionResponse> {
    let subscription = SUBSCRIPTIONS.load(deps.storage, subscription_id)?;

    Ok(SubscriptionResponse {
        id: subscription_id,
        data: subscription,
    })
}

fn query_user_subscriptions(
    deps: Deps,
    user_address: String,
    start_after: Option<u64>,
    limit: Option<u8>,
) -> StdResult<Vec<SubscriptionResponse>> {
    // Validate user address
    let user_addr = deps.api.addr_validate(&user_address)?;
    let limit = limit.unwrap_or(20) as usize;
    let start = start_after.map(Bound::exclusive);

    // Load user subscriptions
    let subscriptions = USER_SUBSCRIPTIONS
        .prefix(user_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, subscription_id) = item?;
            let subscription = SUBSCRIPTIONS.load(deps.storage, subscription_id)?;
            Ok(SubscriptionResponse {
                id: subscription_id,
                data: subscription,
            })
        })
        .collect::<StdResult<Vec<SubscriptionResponse>>>()?;

    Ok(subscriptions)
}

fn query_subscription_plan_subscriptions(
    deps: Deps,
    plan_id: u64,
    start_after: Option<String>,
    limit: Option<u8>,
) -> StdResult<Vec<SubscriptionResponse>> {
    let limit = limit.unwrap_or(20) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.map(Bound::exclusive);

    // Load subscription plan subscriptions
    // Load subscriptions for each subscription id
    let subscriptions = SUBSCRIPTION_PLAN_SUBSCRIPTIONS
        .prefix(plan_id)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, subscription_id) = item?;
            let subscription = SUBSCRIPTIONS.load(deps.storage, subscription_id)?;
            Ok(SubscriptionResponse {
                id: subscription_id,
                data: subscription,
            })
        })
        .collect::<StdResult<Vec<SubscriptionResponse>>>()?;

    Ok(subscriptions)
}

fn query_is_subscribed(
    deps: Deps,
    env: Env,
    user_address: String,
    plan_id: u64,
) -> StdResult<bool> {
    // Validate user address
    let user_addr = deps.api.addr_validate(&user_address)?;

    // Load subscription
    let subscription = SUBSCRIPTIONS.load(deps.storage, plan_id)?;

    // Check if user is subscribed to the plan
    let is_subscribed = subscription.subscriber == user_addr;

    // Check if the subscription is canceled
    let is_canceled = subscription.canceled;

    // Check if the subscription is expired
    let is_expired = subscription.expiration < env.block.time;

    Ok(is_subscribed && !is_canceled && !is_expired)
}
