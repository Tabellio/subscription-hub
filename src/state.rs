use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::BTreeMap;

#[cw_serde]
pub struct Organization {
    // The address of the organization's owner
    pub owner: Addr,
    // Name of the organization
    pub name: String,
    // Description of the organization
    pub description: String,
    // Website of the organization
    pub website: Option<String>,
    // Key-Value metadata of the organization
    pub metadata: Option<BTreeMap<String, String>>,
}

// The organization_id is the primary key for the organization
const ORGANIZATION_ID: Item<u32> = Item::new("organization_id");

// The organizations map stores the organization_id -> organization
const ORGANIZATIONS: Map<u32, Organization> = Map::new("organizations");

// The user_organizations map stores the user -> organization_id
const USER_ORGANIZATIONS: Map<Addr, Vec<u32>> = Map::new("user_organizations");

#[cw_serde]
pub struct SubscriptionPlan {
    // The address of the organization that owns the subscription plan
    pub organization_id: u32,
    // Name of the subscription plan
    pub name: String,
    // Description of the subscription plan
    pub description: String,
    // Price of the subscription plan
    pub payment_amount: Uint128,
    // Expiration time of the subscription plan
    pub expiration: Timestamp,
    // List of features of the subscription plan
    pub features: Option<Vec<String>>,
    // Key-Value metadata of the subscription plan
    pub metadata: Option<BTreeMap<String, String>>,
    // Whether the subscription plan is cancelable
    pub cancelable: bool,
    // Whether the subscription plan is refundable
    pub refundable: bool,
}

// The subscription_plan_id is the primary key for the subscription plan
const SUBSCRIPTION_PLAN_ID: Item<u64> = Item::new("plan_id");

// The subscription_plans map stores the subscription_plan_id -> subscription_plan
const SUBSCRIPTION_PLANS: Map<u64, SubscriptionPlan> = Map::new("subscription_plans");

// The organization_subscription_plans map stores the organization_id -> subscription_plan_id
const ORGANIZATION_SUBSCRIPTION_PLANS: Map<u32, Vec<u64>> =
    Map::new("organization_subscription_plans");
