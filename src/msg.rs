use std::collections::BTreeMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};

use crate::state::{Organization, Subscription, SubscriptionPlan};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrganization {
        name: String,
        description: String,
        website: Option<String>,
        metadata: Option<Vec<(String, String)>>,
    },
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
    SubscribePlan {
        plan_id: u64,
    },
    CancelPlan {
        plan_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Organization)]
    Organization { organization_id: u32 },
    #[returns(Vec<Organization>)]
    UserOrganizations { user_address: String },
    #[returns(SubscriptionPlan)]
    SubscriptionPlan { plan_id: u64 },
    #[returns(Vec<SubscriptionPlan>)]
    SubscriptionPlans { organization_id: u32 },
    #[returns(Subscription)]
    Subscription { subscription_id: u64 },
    #[returns(Vec<Subscription>)]
    UserSubscriptions { user_address: String },
    #[returns(Vec<Subscription>)]
    SubscriptionPlanSubscriptions { plan_id: u64 },
}
