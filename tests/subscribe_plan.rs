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

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, false);

    app.execute_contract(
        Addr::unchecked(USER),
        subscription_hub.clone(),
        &ExecuteMsg::SubscribePlan { plan_id: 1 },
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
    assert_eq!(res.data.subscriber, USER);
    assert_eq!(res.data.plan_id, 1);
    assert_eq!(
        res.data.expiration,
        app.block_info().time.plus_seconds(2592000)
    );

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
    assert_eq!(res, true);

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
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].data.subscriber, USER);
    assert_eq!(res[0].data.plan_id, 1);
}

#[test]
fn test_existing_subscription() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1, false);

    app.execute_contract(
        Addr::unchecked(USER),
        subscription_hub.clone(),
        &ExecuteMsg::SubscribePlan { plan_id: 1 },
        &vec![],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            subscription_hub.clone(),
            &ExecuteMsg::SubscribePlan { plan_id: 1 },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::AlreadySubscribed {}.to_string()
    )
}
