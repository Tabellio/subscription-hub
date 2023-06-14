pub mod helpers;
use helpers::*;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use subscription_hub::{
    msg::{ExecuteMsg, QueryMsg},
    state::Subscription,
};

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    create_organization(&mut app, &subscription_hub, ORGANIZATION);

    create_subscription_plan(&mut app, &subscription_hub, ORGANIZATION, 1);

    app.execute_contract(
        Addr::unchecked(USER),
        subscription_hub.clone(),
        &ExecuteMsg::SubscribePlan { plan_id: 1 },
        &vec![],
    )
    .unwrap();

    let res: Subscription = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::Subscription { subscription_id: 1 },
        )
        .unwrap();
    assert_eq!(res.subscriber, USER);
    assert_eq!(res.plan_id, 1);
    assert_eq!(res.expiration, app.block_info().time.plus_seconds(2592000));

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
}
