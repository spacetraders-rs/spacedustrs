# spacedustrs
This is a rust API wrapper for https://spacetraders.io V2

Many of the client functions in this project are based on, or in some cases copied directly from, the work of https://github.com/bloveless/spacetraders-rs

## Quickstart

spacedustrs is currently not a published crate, however, if you'd like to use it locally in the meantime, this should get you started:

```rust
use spacedustrs::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Setup Game Client
    let http_client = spacedustrs::client::get_http_client(None);

    // Register agent
    let claim_agent_response = spacedustrs::client::claim_agent(
        http_client,
        "<4-8 character string>".to_string(),
        "COMMERCE_REPUBLIC".to_string(),
    )
    .await.unwrap();

    let client = Client::new(
        http_client,
        claim_agent_response.data.agent.symbol,
        claim_agent_response.token,
    );

    match client.get_my_agent_details().await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(res_err) => {
            println!("{:#?}", res_err);
        }
    }

    Ok(())
}
```

## Supported Endpoints

 - `POST: /agents`
 - - `spacedustrs::client::claim_agent(http_client: http_client, agent_symbol: String, agent_faction: String) -> responses::ClaimAgent`
 - `GET: /my/agent`
 - - `Client.get_my_agent_details() -> responses::AgentDetails`
