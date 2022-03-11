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
        "https://v2-0-0.alpha.spacetraders.io".to_string(),
        "<4-8 character string>".to_string(),
        "COMMERCE_REPUBLIC".to_string(),
    )
    .await.unwrap();

    // Setup client using claimed agent
    let client = Client::new(
        http_client,
        "https://v2-0-0.alpha.spacetraders.io".to_string(),
        claim_agent_response.data.agent.symbol,
        claim_agent_response.token,
    );

    // Get agent details to confirm working
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

### Public Endpoints

**Agent & Account**

- `POST` /agents
- - `spacedustrs::client::claim_agent(base_url: String, http_client: http_client, agent_symbol: String, agent_faction: String) -> responses::ClaimAgent`

### Non-Specific Endpoints That Still Require Authentication

**Systems**

- `GET` /systems
- - `Client.get_systems() -> responses::SystemsListResponse`
- `GET` /systems/{systemSymbol}
- - `Client.get_system_info(system_symbol: String) -> responses::SystemInformationResponse`

### Agent Specific Endpoints

**Agent**

- `GET` /my/agent
- - `Client.get_my_agent_details() -> responses::AgentDetails`

**Contracts**

- `GET` /my/contracts
- - `Client.get_my_contracts() -> responses::ContractsResponse`
- `GET` /my/contracts/{contractId}
- - `Client.get_my_contract(contract_id: String) -> responses::ContractResponse`
- `POST` /my/contracts/{contractId}/accept
- - `Client.get_my_contract(contract_id: String) -> responses::AcceptedContractResponse`

**Ships**

- `GET` /my/ships
- - `Client.get_my_ships() -> responses::ShipsResponse`
- `GET` /my/ships/{shipSymbol}
- - `Client.get_my_ship(ship_id: String) -> responses::ShipResponse`
- `GET` /my/ships/{shipSymbol}/navigate
- - `Client.ship_navigation_status(ship_id: String) -> responses::NavigateResponse`
- `POST` /my/ships/{shipSymbol}/navigate destination=destination_symbol
- - `Client.ship_navigation_status(ship_id: String, destination_symbol: String) -> responses::NavigateResponse`
- `GET` /my/ships/{shipSymbol}/survey
- - `Client.get_survey_cooldown(ship_id: String) -> responses::SurveyCooldownResponse`
- `POST` /my/ships/{shipSymbol}/survey
- - `Client.survey_surroundings(ship_id: String) -> responses::SurveyResponse`
- `GET` /my/ships/{shipSymbol}/extract
- - `Client.get_extract_cooldown(ship_id: String) -> responses::ExtractCooldownResponse`
- `POST` /my/ships/{shipSymbol}/extract ?survey{}=survey
- - `Client.extract_resources(ship_id: String, survey: Option<Survey>) -> responses::ExtractResourcesResponse`
- `POST` /my/ships/{shipSymbol}/dock
- - `Client.dock_ship(ship_id: String) -> responses::StatusResponse`
- `POST` /my/ships/{shipSymbol}/orbit
- - `Client.orbit_ship(ship_id: String) -> responses::StatusResponse`
- `POST` /my/ships/{shipSymbol}/deliver
- - `Client.deliver_goods(ship_id: String, contract_id: String, trade_symbol: String, units: u64) -> responses::DeliveryResponse`

## Unsupported Endpoints

- `POST` /my/ships/{shipSymbol}/jettison symbol=HEAVY_MACHINERY quantity=99999
- - ``
- `POST` /my/ships/{shipSymbol}/refuel
- - ``
- `POST` /my/ships/{shipSymbol}/scan
- - ``
- `POST` /my/ships/{shipSymbol}/jump destination=X1-OE # jump to a target system
- - ``
- `POST` /my/ships/{shipSymbol}/purchase symbol=HEAVY_MACHINERY quantity=99999
- - ``
- `POST` /my/ships/{shipSymbol}/sell symbol=HEAVY_MACHINERY quantity=99999
- - ``
- `GET` /systems/X1-OE/waypoints/X1-OE-001A # view waypoint details
- - ``
- `GET` /systems/X1-OE/shipyards # view all shipyards in a system
- - ``
- `GET` /systems/X1-OE/shipyards/X1-OE-001A/ships # view all ships for sell at a shipyard
- - ``
- `GET` /systems/X1-OE/markets # view all markets in a system
- - ``
- `GET` /systems/X1-OE/markets/X1-OE-001A # view all trades at a given market
- - ``

## Notes from Roadmap

- `/navigate` will support modes
- `/dock` will support modes
- `/orbit` will support modes
- `GET` /my/account
- `POST` /my/ships/{shipSymbol}/chart # appears to be incomplete