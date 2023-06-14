use cosmwasm_std::{Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
use subscription_hub::{
    msg::{ExecuteMsg, InstantiateMsg},
    state::DurationUnit,
};

pub fn subscription_hub() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        subscription_hub::contract::execute,
        subscription_hub::contract::instantiate,
        subscription_hub::contract::query,
    );
    Box::new(contract)
}

pub const ADMIN: &str = "admin";

pub const ORGANIZATION: &str = "organization";
pub const ORGANIZATION2: &str = "organization2";
pub const ORGANIZATION3: &str = "organization3";

pub const USER: &str = "user";
pub const USER2: &str = "user2";
pub const USER3: &str = "user3";

pub const DENOM: &str = "uconst";

pub fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ADMIN),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ORGANIZATION),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ORGANIZATION2),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ORGANIZATION3),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(USER),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(USER2),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(USER3),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
    })
}

pub fn proper_instantiate(app: &mut App, admin: &str) -> Addr {
    let code_id = app.store_code(subscription_hub());
    app.instantiate_contract(
        code_id,
        Addr::unchecked(admin),
        &InstantiateMsg {},
        &vec![],
        "CosmWasm Subscription Hub",
        None,
    )
    .unwrap()
}

pub fn create_organization(app: &mut App, subscription_hub: &Addr, owner: &str) {
    app.execute_contract(
        Addr::unchecked(owner),
        subscription_hub.clone(),
        &ExecuteMsg::CreateOrganization {
            name: "Test Organization".to_string(),
            description: "Test organization is the best".to_string(),
            website: None,
            metadata: None,
        },
        &vec![],
    )
    .unwrap();
}

pub fn create_subscription_plan(
    app: &mut App,
    subscription_hub: &Addr,
    owner: &str,
    organization_id: u32,
) {
    app.execute_contract(
        Addr::unchecked(owner),
        subscription_hub.clone(),
        &ExecuteMsg::CreateSubscriptionPlan {
            organization_id,
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
}
