pub mod helpers;
use helpers::*;

use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;
use subscription_hub::{
    msg::{ExecuteMsg, QueryMsg, SubscriptionPlanResponse},
    state::DurationUnit,
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

    let res: SubscriptionPlanResponse = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 1 },
        )
        .unwrap();
    assert_eq!(res.data.name, "Test Plan");
    assert_eq!(res.data.description, "Test plan is the best");
    assert_eq!(res.data.price, Uint128::new(10_000));
    assert_eq!(res.data.duration, 1);
    assert_eq!(res.data.duration_unit, DurationUnit::Month);

    let res: SubscriptionPlanResponse = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 2 },
        )
        .unwrap();
    assert_eq!(res.data.name, "Second Test Plan");
    assert_eq!(res.data.description, "Second test plan is the best");
    assert_eq!(res.data.price, Uint128::new(50_000));
    assert_eq!(res.data.duration, 6);
    assert_eq!(res.data.duration_unit, DurationUnit::Month);

    let res: SubscriptionPlanResponse = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlan { plan_id: 3 },
        )
        .unwrap();
    assert_eq!(res.data.name, "Third Test Plan");
    assert_eq!(res.data.description, "Third test plan is the best");
    assert_eq!(res.data.price, Uint128::new(250_000));
    assert_eq!(res.data.duration, 1);
    assert_eq!(res.data.duration_unit, DurationUnit::Year);
    assert_eq!(res.data.features.unwrap().len(), 8);

    let res: Vec<SubscriptionPlanResponse> = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::OrganizationSubscriptionPlans { organization_id: 1 },
        )
        .unwrap();
    assert_eq!(res.len(), 3);
}

#[test]
fn test_invalid_organization_owner() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    // Create subscription plan with invalid organization owner
    let err = app
        .execute_contract(
            Addr::unchecked(ORGANIZATION2),
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
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        "Unauthorized".to_string()
    );
}
