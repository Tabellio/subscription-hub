pub mod helpers;
use helpers::*;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use subscription_hub::{
    msg::{ExecuteMsg, QueryMsg, SubscriptionResponse},
    ContractError,
};

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, true);

    subscribe_plan(&mut app, &subscription_hub, USER, 1);

    app.execute_contract(
        Addr::unchecked(USER),
        subscription_hub.clone(),
        &ExecuteMsg::CancelPlan { plan_id: 1 },
        &vec![],
    )
    .unwrap();

    let res: SubscriptionResponse = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::Subscription { subscription_id: 1 },
        )
        .unwrap();
    assert_eq!(res.data.canceled, true);

    let res: bool = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::IsSubscribed {
                user_address: USER.to_string(),
                plan_id: 1,
            },
        )
        .unwrap();
    assert_eq!(res, false);

    let res: Vec<SubscriptionResponse> = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::SubscriptionPlanSubscriptions {
                plan_id: 1,
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(res.len(), 0);
}

#[test]
fn test_invalid_subscriber() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, true);

    subscribe_plan(&mut app, &subscription_hub, USER, 1);

    let err = app
        .execute_contract(
            Addr::unchecked(USER2),
            subscription_hub.clone(),
            &ExecuteMsg::CancelPlan { plan_id: 1 },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::Unauthorized {}.to_string()
    );
}

#[test]
fn test_non_cancelable_subscription_plan() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, false);

    subscribe_plan(&mut app, &subscription_hub, USER, 1);

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            subscription_hub.clone(),
            &ExecuteMsg::CancelPlan { plan_id: 1 },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::NotCancelable {}.to_string()
    );
}

#[test]
fn test_already_canceled_subscription() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, true);

    subscribe_plan(&mut app, &subscription_hub, USER, 1);

    app.execute_contract(
        Addr::unchecked(USER),
        subscription_hub.clone(),
        &ExecuteMsg::CancelPlan { plan_id: 1 },
        &vec![],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            subscription_hub.clone(),
            &ExecuteMsg::CancelPlan { plan_id: 1 },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::AlreadyCanceled {}.to_string()
    );
}

#[test]
fn test_already_expired_subscription() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, true);

    subscribe_plan(&mut app, &subscription_hub, USER, 1);

    // Move time forward by 30 days and 1 second
    app.update_block(|block| block.time = block.time.plus_seconds(2592001));

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            subscription_hub.clone(),
            &ExecuteMsg::CancelPlan { plan_id: 1 },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::AlreadyExpired {}.to_string()
    );
}
