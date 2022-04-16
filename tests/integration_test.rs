// use spacedust::errors::*;
// use spacedust::responses::*;
use spacedust::shared::*;

use tokio::time::{sleep, Duration};

mod common;

use crate::common::TestClient;

// TODO: each test should create its own agent and check several functions for its lifetime, allowing multiple threads to work simultaneously and ensuring state for state-dependent endpoints

// #[ignore]
// #[tokio::test]
// async fn test_claim_agent() {
//     let tc = TestClient::new();
//     // TODO: delete agent then recreate
//     match spacedust::client::claim_agent(tc.http_client, tc.client.base_url, tc.client.agentname, tc.client.token).await {
//         Ok(_) => assert!(true),
//         Err(testerr) => {
//             println!("ERROR: {:?}", testerr);
//             assert!(false);
//         }
//     }
// }

#[tokio::test]
async fn test_get_my_agent_details() {
    let tc = TestClient::new();
    match tc.client.get_my_agent_details().await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_get_my_contracts() {
    let tc = TestClient::new();
    match tc.client.get_my_contracts().await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_get_my_contract() {
    let tc = TestClient::new();
    // Dynmically get from list to test with
    let test_target: String = tc.client.get_my_contracts().await.unwrap().data[0]
        .id
        .to_string();
    match tc.client.get_my_contract(test_target).await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[ignore]
#[tokio::test]
async fn test_accept_my_contract() {
    let tc = TestClient::new();
    // Dynmically get from list to test with
    let test_target: String = tc.client.get_my_contracts().await.unwrap().data[0]
        .id
        .to_string();
    match tc.client.accept_my_contract(test_target).await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_get_my_ships() {
    let tc = TestClient::new();
    match tc.client.get_my_ships().await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_get_my_ship() {
    let tc = TestClient::new();
    // Dynmically get from list to test with
    let test_target: String = tc.client.get_my_ships().await.unwrap().data[0]
        .symbol
        .to_string();
    match tc.client.get_my_ship(test_target).await {
        Ok(_) => assert!(true),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_navigate_refuel_survey_extract_dock_orbit() {
    env_logger::init();

    let tc = TestClient::new();
    // Dynmically get from list to test with
    let test_target: String = tc.client.get_my_ships().await.unwrap().data[0]
        .symbol
        .to_string();
    let mut delay: Duration;
    // Tests navigate ship
    match tc
        .client
        .navigate_ship(test_target.to_string(), "X1-OE-PM".to_string())
        .await
    {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    // Tests ship_navigation_status
    match tc
        .client
        .ship_navigation_status(test_target.to_string())
        .await
    {
        Ok(res) => {
            delay = Duration::from_millis(res.data.navigation.duration_remaining.unwrap() + 1)
        }
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
            return;
        }
    }
    sleep(delay * 1000).await;
    match tc.client.refuel_ship(test_target.to_string()).await {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    match tc
        .client
        .navigate_ship(test_target.to_string(), "X1-OE-A005".to_string())
        .await
    {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    // Tests ship_navigation_status
    match tc
        .client
        .ship_navigation_status(test_target.to_string())
        .await
    {
        Ok(res) => {
            delay = Duration::from_millis(res.data.navigation.duration_remaining.unwrap() + 1)
        }
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
            return;
        }
    }
    sleep(delay * 1000).await;
    let mut surveys: Vec<Survey>;
    match tc.client.survey_surroundings(test_target.to_string()).await {
        Ok(res) => surveys = res.data.surveys,
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
            return;
        }
    }
    match tc.client.get_survey_cooldown(test_target.to_string()).await {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    // Test extract with survey
    match tc
        .client
        .extract_resources(test_target.to_string(), Some(surveys.remove(0)))
        .await
    {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    match tc
        .client
        .get_extract_cooldown(test_target.to_string())
        .await
    {
        Ok(res) => (delay = Duration::from_millis(res.data.cooldown.duration + 1)),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
            return;
        }
    }
    sleep(delay * 1000).await;
    // Test extract without survey
    match tc
        .client
        .extract_resources(test_target.to_string(), None)
        .await
    {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    match tc.client.dock_ship(test_target.to_string()).await {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
    match tc.client.orbit_ship(test_target.to_string()).await {
        Ok(_) => (),
        Err(testerr) => {
            println!("ERROR: {:?}", testerr);
            assert!(false);
        }
    }
}

// deliver_goods
// refuel_ship
