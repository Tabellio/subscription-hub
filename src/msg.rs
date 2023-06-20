use std::collections::BTreeMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::state::{DurationUnit, Organization, Subscription, SubscriptionPlan};

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
    #[returns(OrganizationResponse)]
    Organization { organization_id: u32 },
    // Get all organizations owned by the given user
    #[returns(Vec<OrganizationResponse>)]
    UserOrganizations { user_address: String },
    // Get the subscription plan with the given ID
    #[returns(SubscriptionPlanResponse)]
    SubscriptionPlan { plan_id: u64 },
    // Get all subscription plans owned by the given organization
    #[returns(Vec<SubscriptionPlanResponse>)]
    OrganizationSubscriptionPlans { organization_id: u32 },
    // Get the subscription with the given ID
    #[returns(SubscriptionResponse)]
    Subscription { subscription_id: u64 },
    // Get all subscriptions owned by the given user
    #[returns(Vec<SubscriptionResponse>)]
    UserSubscriptions {
        user_address: String,
        start_after: Option<u64>,
        limit: Option<u8>,
    },
    // Get all subscriptions for the given subscription plan
    #[returns(Vec<SubscriptionResponse>)]
    SubscriptionPlanSubscriptions {
        plan_id: u64,
        start_after: Option<String>,
        limit: Option<u8>,
    },
    // Checks if the given user is subscribed to the given subscription plan
    #[returns(bool)]
    IsSubscribed { user_address: String, plan_id: u64 },
}

#[cw_serde]
pub struct OrganizationResponse {
    pub id: u32,
    pub data: Organization,
}

#[cw_serde]
pub struct SubscriptionPlanResponse {
    pub id: u64,
    pub data: SubscriptionPlan,
}

#[cw_serde]
pub struct SubscriptionResponse {
    pub id: u64,
    pub data: Subscription,
}
