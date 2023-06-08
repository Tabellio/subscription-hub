use std::collections::BTreeMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};

use crate::state::{Organization, Subscription, SubscriptionPlan};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // Create a new organization
    CreateOrganization {
        name: String,
        description: String,
        website: Option<String>,
        metadata: Option<BTreeMap<String, String>>,
    },
    // Create a new subscription plan for an organization
    CreateSubscriptionPlan {
        name: String,
        description: String,
        payment_amount: Uint128,
        expiration: Timestamp,
        features: Option<Vec<String>>,
        metadata: Option<BTreeMap<String, String>>,
        cancelable: bool,
        refundable: bool,
    },
    // Subscribe to a subscription plan
    SubscribePlan {
        plan_id: u64,
    },
    // Cancel a subscription plan
    CancelPlan {
        plan_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Get the organization with the given ID
    #[returns(Organization)]
    Organization { organization_id: u32 },
    // Get all organizations owned by the given user
    #[returns(Vec<Organization>)]
    UserOrganizations { user_address: String },
    // Get the subscription plan with the given ID
    #[returns(SubscriptionPlan)]
    SubscriptionPlan { plan_id: u64 },
    // Get all subscription plans owned by the given organization
    #[returns(Vec<SubscriptionPlan>)]
    SubscriptionPlans { organization_id: u32 },
    // Get the subscription with the given ID
    #[returns(Subscription)]
    Subscription { subscription_id: u64 },
    // Get all subscriptions owned by the given user
    #[returns(Vec<Subscription>)]
    UserSubscriptions { user_address: String },
    // Get all subscriptions for the given subscription plan
    #[returns(Vec<Subscription>)]
    SubscriptionPlanSubscriptions { plan_id: u64 },
}
