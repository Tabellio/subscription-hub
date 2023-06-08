use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    // The address of the contract admin
    pub admin: Addr,
}

// The config key is used to store the contract's configuration
pub const CONFIG: Item<Config> = Item::new("config");

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
pub const ORGANIZATION_ID: Item<u32> = Item::new("organization_id");

// The organizations map stores the organization_id -> organization
pub const ORGANIZATIONS: Map<u32, Organization> = Map::new("organizations");

// The user_organizations map stores the user -> organization_id
pub const USER_ORGANIZATIONS: Map<Addr, Vec<u32>> = Map::new("user_organizations");

#[cw_serde]
pub enum DurationUnit {
    Day,
    Week,
    Month,
    Year,
}

#[cw_serde]
pub struct SubscriptionPlan {
    // The address of the organization that owns the subscription plan
    pub organization_id: u32,
    // Name of the subscription plan
    pub name: String,
    // Description of the subscription plan
    pub description: String,
    // Price of the subscription plan
    pub price: Uint128,
    // Duration of the subscription plan
    pub duration: u8,
    // Unit of the duration of the subscription plan
    pub duration_unit: DurationUnit,
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
pub const SUBSCRIPTION_PLAN_ID: Item<u64> = Item::new("plan_id");

// The subscription_plans map stores the subscription_plan_id -> subscription_plan
pub const SUBSCRIPTION_PLANS: Map<u64, SubscriptionPlan> = Map::new("subscription_plans");

// The organization_subscription_plans map stores the organization_id -> subscription_plan_id
pub const ORGANIZATION_SUBSCRIPTION_PLANS: Map<u32, Vec<u64>> =
    Map::new("organization_subscription_plans");

#[cw_serde]
pub struct Subscription {
    // The address of the subscriber
    pub subscriber: Addr,
    // Subscription plan id
    pub plan_id: u64,
    // Subscription expiration
    pub expiration: Timestamp,
    // Whether the subscription is canceled
    pub canceled: bool,
}

// The subscription_id is the primary key for the subscription
pub const SUBSCRIPTION_ID: Item<u64> = Item::new("subscription_id");

// The subscriptions map stores the subscription_id -> subscription
pub const SUBSCRIPTIONS: Map<u64, Subscription> = Map::new("subscriptions");

// The subscriber_subscriptions map stores the subscriber -> subscription_id
pub const SUBSCRIBER_SUBSCRIPTIONS: Map<Addr, Vec<u64>> = Map::new("subscriber_subscriptions");

// The subscription_plan_subscriptions map stores the subscription_plan_id and subscriber -> subscription_id
pub const SUBSCRIPTION_PLAN_SUBSCRIPTIONS: Map<(u64, Addr), u64> =
    Map::new("subscription_plan_subscriptions");
