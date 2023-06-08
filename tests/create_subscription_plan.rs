pub mod helpers;
use helpers::*;

use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;
use subscription_hub::{
    msg::{ExecuteMsg, QueryMsg},
    state::{DurationUnit, SubscriptionPlan},
};

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    // Create the first subscription plan
    app.execute_contract(
        Addr::unchecked(ORGANIZATION),
        subscription_hub.clone(),
        &ExecuteMsg::CreateSubscriptionPlan {
            organization_id: 1,
            name: "Test Plan".to_string(),
            description: "Test plan is the best".to_string(),
            price: Uint128::new(10_000),
            duration: 1,
            duration_unit: DurationUnit::Month,
            features: Some(vec![
                "first_feature".to_string(),
                "second_feature".to_string(),
            ]),
            metadata: None,
            cancelable: false,
            refundable: false,
        },
        &vec![],
    )
    .unwrap();

    // Create the second subscription plan
    app.execute_contract(
        Addr::unchecked(ORGANIZATION),
        subscription_hub.clone(),
        &ExecuteMsg::CreateSubscriptionPlan {
            organization_id: 1,
            name: "Second Test Plan".to_string(),
            description: "Second test plan is the best".to_string(),
            price: Uint128::new(50_000),
            duration: 6,
            duration_unit: DurationUnit::Month,
            features: Some(vec![
                "first_feature".to_string(),
                "second_feature".to_string(),
                "third_feature".to_string(),
                "fourth_feature".to_string(),
            ]),
            metadata: None,
            cancelable: false,
            refundable: false,
        },
        &vec![],
    )
    .unwrap();

    // Create the second subscription plan
    app.execute_contract(
        Addr::unchecked(ORGANIZATION),
        subscription_hub.clone(),
        &ExecuteMsg::CreateSubscriptionPlan {
            organization_id: 1,
            name: "Third Test Plan".to_string(),
            description: "Third test plan is the best".to_string(),
            price: Uint128::new(250_000),
            duration: 1,
            duration_unit: DurationUnit::Year,
            features: Some(vec![
                "first_feature".to_string(),
                "second_feature".to_string(),
                "third_feature".to_string(),
                "fourth_feature".to_string(),
                "fifth_feature".to_string(),
                "sixth_feature".to_string(),
                "seventh_feature".to_string(),
                "eighth_feature".to_string(),
            ]),
            metadata: None,
            cancelable: false,
            refundable: false,
        },
        &vec![],
    )
    .unwrap();

    let subscription_plan: SubscriptionPlan = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 1 },
        )
        .unwrap();
    assert_eq!(subscription_plan.name, "Test Plan");
    assert_eq!(subscription_plan.description, "Test plan is the best");
    assert_eq!(subscription_plan.price, Uint128::new(10_000));
    assert_eq!(subscription_plan.duration, 1);
    assert_eq!(subscription_plan.duration_unit, DurationUnit::Month);

    let subscription_plan: SubscriptionPlan = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 2 },
        )
        .unwrap();
    assert_eq!(subscription_plan.name, "Second Test Plan");
    assert_eq!(
        subscription_plan.description,
        "Second test plan is the best"
    );
    assert_eq!(subscription_plan.price, Uint128::new(50_000));
    assert_eq!(subscription_plan.duration, 6);
    assert_eq!(subscription_plan.duration_unit, DurationUnit::Month);

    let subscription_plan: SubscriptionPlan = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 3 },
        )
        .unwrap();
    assert_eq!(subscription_plan.name, "Third Test Plan");
    assert_eq!(subscription_plan.description, "Third test plan is the best");
    assert_eq!(subscription_plan.price, Uint128::new(250_000));
    assert_eq!(subscription_plan.duration, 1);
    assert_eq!(subscription_plan.duration_unit, DurationUnit::Year);
    assert_eq!(subscription_plan.features.unwrap().len(), 8);

    let organization_subscription_plans: Vec<SubscriptionPlan> = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::OrganizationSubscriptionPlans { organization_id: 1 },
        )
        .unwrap();
    assert_eq!(organization_subscription_plans.len(), 3);
}
