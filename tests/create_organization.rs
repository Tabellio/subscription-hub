pub mod helpers;
use helpers::*;

use std::collections::BTreeMap;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use subscription_hub::{
    msg::{ExecuteMsg, QueryMsg},
    state::Organization,
};

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let subscription_hub = proper_instantiate(&mut app, ADMIN);

    app.execute_contract(
        Addr::unchecked(ORGANIZATION),
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

    let mut organization2_metadata = BTreeMap::new();
    organization2_metadata.insert("first_feature".to_string(), "true".to_string());
    organization2_metadata.insert("second_feature".to_string(), "false".to_string());
    app.execute_contract(
        Addr::unchecked(ORGANIZATION),
        subscription_hub.clone(),
        &ExecuteMsg::CreateOrganization {
            name: "Second Test Organization".to_string(),
            description: "Second test organization is the best".to_string(),
            website: None,
            metadata: Some(organization2_metadata.clone()),
        },
        &vec![],
    )
    .unwrap();

    let organization: Organization = app
        .wrap()
        .query_wasm_smart(
            subscription_hub.clone(),
            &QueryMsg::Organization { organization_id: 1 },
        )
        .unwrap();
    assert_eq!(organization.name, "Test Organization");
    assert_eq!(organization.description, "Test organization is the best");
    assert_eq!(organization.website, None);
    assert_eq!(organization.metadata, None);

    let user_organizations: Vec<Organization> = app
        .wrap()
        .query_wasm_smart(
            subscription_hub,
            &QueryMsg::UserOrganizations {
                user_address: ORGANIZATION.to_string(),
            },
        )
        .unwrap();
    assert_eq!(user_organizations.len(), 2);
    assert_eq!(user_organizations[0].name, "Test Organization");
    assert_eq!(
        user_organizations[0].description,
        "Test organization is the best"
    );
    assert_eq!(user_organizations[0].website, None);
    assert_eq!(user_organizations[0].metadata, None);
    assert_eq!(user_organizations[1].name, "Second Test Organization");
    assert_eq!(
        user_organizations[1].description,
        "Second test organization is the best"
    );
    assert_eq!(user_organizations[1].website, None);
    assert_eq!(user_organizations[1].metadata, Some(organization2_metadata));
}
