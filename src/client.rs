//! The client module wraps the interactions between the client and the server
use crate::shared;
use crate::{requests, responses};

use crate::errors::SpaceTradersClientError;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

/// HttpClient is a thread-safe rate-limited space traders client
pub type HttpClient = Arc<Mutex<SpaceTradersClient>>;

/// Allow the user to tie into the request lifecycle and do things with the request, responses, and/or error coming back
pub type PostRequestHook = fn(
    method: &str,
    url: &str,
    request_body: Option<&str>,
    response_status_code: Option<u16>,
    response_headers: Option<&HashMap<String, String>>,
    response_body: Option<&str>,
    error: Option<&SpaceTradersClientError>,
);

/// SpaceTradersClient wraps the actual reqwest client and adds rate-limiting support
#[derive(Clone)]
pub struct SpaceTradersClient {
    client: reqwest::Client,
    post_request_hook: Option<PostRequestHook>,
}

impl Debug for SpaceTradersClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpaceTradersClient")
            .field("client", &self.client)
            .finish()
    }
}

/// SpaceTradersClientRequest wraps all the parameters sent to the spacetraders client
#[derive(Serialize)]
pub struct SpaceTradersClientRequest {
    method: String,
    url: String,
    request_headers: HashMap<String, String>,
    request_text: String,
}

/// SpaceTradersClientRequest wraps all the parameters received from the spacetraders API
#[derive(Serialize)]
pub struct SpaceTradersClientResponse {
    status_code: u16,
    response_headers: HashMap<String, String>,
    response_text: String,
}

impl SpaceTradersClient {
    fn new(proxy: Option<String>) -> Self {
        let mut client_builder = reqwest::ClientBuilder::new();

        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }

        Self {
            client: client_builder.build().unwrap(),
            post_request_hook: None,
        }
    }

    fn set_post_request_hook(&mut self, hook: PostRequestHook) {
        self.post_request_hook = Some(hook);
    }

    async fn execute_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        token: Option<&str>,
    ) -> Result<SpaceTradersClientResponse, SpaceTradersClientError> {
        let mut request_builder = self
            .client
            .request(Method::from_str(&method).unwrap(), Url::parse(url).unwrap());

        if let Some(token) = token {
            request_builder = request_builder.header(
                HeaderName::from_lowercase(b"authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap(),
            );
        }

        if let Some(body) = body {
            request_builder = request_builder.header(
                HeaderName::from_lowercase(b"content-type").unwrap(),
                HeaderValue::from_static("application/json"),
            );
            request_builder = request_builder.body(body.to_owned());
        } else if method == "POST" {
            // If body empty, set content-length to 0
            request_builder = request_builder.header(
                HeaderName::from_lowercase(b"content-length").unwrap(),
                HeaderValue::from_static("0"),
            );
        }

        let mut attempts = 0;
        let request = request_builder.build().unwrap();
        loop {
            attempts += 1;
            if attempts > 3 {
                return Err(SpaceTradersClientError::TooManyRetries);
            }

            match self.client.execute(request.try_clone().unwrap()).await {
                Ok(response) => {
                    let response_headers =
                        response
                            .headers()
                            .iter()
                            .fold(HashMap::new(), |mut acc, (h, v)| {
                                acc.insert(h.to_string(), v.to_str().unwrap().to_string());
                                acc
                            });
                    let response_status = response.status();
                    let response_text = response.text().await?;

                    if let Some(post_request_hook) = self.post_request_hook {
                        post_request_hook(
                            method,
                            url,
                            body,
                            Some(response_status.as_u16()),
                            Some(&response_headers),
                            Some(&response_text),
                            None,
                        );
                    }

                    // Check if the response was a throttle exception (status 429 means we have been rate limited)
                    if response_status == 429 {
                        let retry_after: f64 = response_headers
                            .get("retry-after")
                            .unwrap_or(&"1.0".to_string())
                            .parse()
                            .unwrap_or(1.0);

                        // If it was a throttle then wait based on the retry-after response headers
                        log::warn!("Rate limited... waiting for {} seconds before trying again. Request: \"{} {}\"", retry_after, request.method(), request.url());
                        tokio::time::sleep(Duration::from_secs_f64(retry_after)).await;

                        continue;
                    } else if response_status == 401 {
                        return Err(SpaceTradersClientError::Unauthorized);
                    } else if response_status == 500 {
                        // If there was an internal server error then try the request again in 2 seconds
                        log::error!(
                            "Caught internal server error retrying in 2 seconds. {}",
                            response_text
                        );
                        tokio::time::sleep(Duration::from_secs(2)).await;

                        continue;
                    } else {
                        return Ok(SpaceTradersClientResponse {
                            status_code: response_status.as_u16(),
                            response_headers,
                            response_text,
                        });
                    }
                }
                Err(e) => {
                    let space_traders_client_error = SpaceTradersClientError::Http(e);

                    if let Some(post_request_hook) = self.post_request_hook {
                        post_request_hook(
                            method,
                            url,
                            body,
                            None,
                            None,
                            None,
                            Some(&space_traders_client_error),
                        );
                    }

                    return Err(space_traders_client_error);
                }
            }
        }
    }
}

/// Get a rate-limited http client that is safe to use across threads and won't break rate-limiting
pub fn get_http_client(proxy: Option<String>) -> HttpClient {
    Arc::new(Mutex::new(SpaceTradersClient::new(proxy)))
}

/// Get a rate-limited http client, with post receive hook, that is safe to use across threads and
/// won't break rate-limiting
pub fn get_http_client_with_hook(proxy: Option<String>, hook: PostRequestHook) -> HttpClient {
    let mut client = SpaceTradersClient::new(proxy);
    client.set_post_request_hook(hook);

    Arc::new(Mutex::new(client))
}

/// Parse a response string into the type represented by T
/// If the `response_text` cannot be parsed into type T then it is assumed that an error
/// occurred and an shared::ErrorMessage will be returned
///
/// # Arguments
///
/// * `response_text` - A string containing the JSON response to be parsed
fn parse_response<'a, T: Deserialize<'a>>(
    response_text: &'a str,
) -> Result<T, SpaceTradersClientError> {
    match serde_json::from_str::<T>(&response_text) {
        Ok(o) => Ok(o),
        Err(e) => {
            log::error!(
                "Error processing type {:?}: {}",
                std::any::type_name::<T>(),
                e
            );
            log::error!("Error response: {}", &response_text);

            match serde_json::from_str::<shared::ErrorMessage>(&response_text) {
                Ok(error_message) => Err(SpaceTradersClientError::ApiError(error_message)),
                Err(e) => Err(SpaceTradersClientError::JsonParse(e)),
            }
        }
    }
}

/// Claim an agent and get a token
///
/// # Arguments
///
/// * `agent_symbol` - A 4-8 character string containing the username to get a token for
/// * `agent_faction` - Use COMMERCE_REPUBLIC if you don't know any other factions to choose from
pub async fn claim_agent(
    http_client: HttpClient,
    base_url: String,
    agent_symbol: String,
    agent_faction: String,
) -> Result<responses::ClaimAgent, SpaceTradersClientError> {
    let claim_agent_request = requests::ClaimAgentRequest {
        symbol: agent_symbol.clone(),
        faction: agent_faction.clone(),
    };

    let http_client = http_client.lock().await;
    let response = http_client
        .execute_request(
            "POST",
            &format!("{}/agents", base_url),
            Some(&serde_json::to_string(&claim_agent_request).unwrap()),
            None,
        )
        .await?;

    parse_response::<responses::ClaimAgent>(&response.response_text)
}

/// A SpaceTraders client that is associated to a specific username
#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    base_url: String,
    /// The agent's name/symbol
    pub agentname: String,
    /// The uses access token
    pub token: String,
}

impl Client {
    /// Create a new game with a reqwest client that has the Authorization header set
    ///
    /// # Arguments
    ///
    /// * `token` - A string containing the access token for the username provided
    pub fn new(
        http_client: HttpClient,
        base_url: String,
        agentname: String,
        token: String,
    ) -> Client {
        Client {
            http_client,
            base_url,
            agentname,
            token,
        }
    }

    //////////////////////////////////////////////
    ///// AGENT
    //////////////////////////////////////////////
    /// Get agent information
    pub async fn get_my_agent_details(
        &self,
    ) -> Result<responses::AgentDetails, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/agent", &self.base_url),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::AgentDetails>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// CONTRACTS
    //////////////////////////////////////////////
    /// Get list of my contracts
    pub async fn get_my_contracts(
        &self,
    ) -> Result<responses::ContractsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/contracts", &self.base_url),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ContractsResponse>(&response.response_text)
    }

    /// Get info on specific contract
    pub async fn get_my_contract(
        &self,
        contract_id: String,
    ) -> Result<responses::ContractResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/contracts/{}", &self.base_url, contract_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ContractResponse>(&response.response_text)
    }

    /// Accept a specific contract
    pub async fn accept_my_contract(
        &self,
        contract_id: String,
    ) -> Result<responses::AcceptedContractResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/contracts/{}/accept", &self.base_url, contract_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::AcceptedContractResponse>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// SHIPS
    //////////////////////////////////////////////
    /// Get list of my ships
    pub async fn get_my_ships(&self) -> Result<responses::ShipsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships", &self.base_url),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ShipsResponse>(&response.response_text)
    }

    /// Get info on specific ship
    pub async fn get_my_ship(
        &self,
        ship_id: String,
    ) -> Result<responses::ShipResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ShipResponse>(&response.response_text)
    }

    /// Navigate specific ship to target location
    pub async fn navigate_ship(
        &self,
        ship_id: String,
        destination_symbol: String,
    ) -> Result<responses::NavigateResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let navigate_request = requests::NavigateRequest {
            destination: destination_symbol.clone(),
        };

        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/navigate", &self.base_url, ship_id),
                Some(&serde_json::to_string(&navigate_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::NavigateResponse>(&response.response_text)
    }

    /// Get the status of the specified ship's last navigation
    pub async fn ship_navigation_status(
        &self,
        ship_id: String,
    ) -> Result<responses::NavigateInfoResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}/navigate", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::NavigateInfoResponse>(&response.response_text)
    }

    /// Get the survey cooldown of the given ship
    pub async fn get_survey_cooldown(
        &self,
        ship_id: String,
    ) -> Result<responses::CooldownResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}/survey", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::CooldownResponse>(&response.response_text)
    }

    /// Survey the surroundings of the given ship
    pub async fn survey_surroundings(
        &self,
        ship_id: String,
    ) -> Result<responses::SurveyResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/survey", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SurveyResponse>(&response.response_text)
    }

    /// Get the extraction cooldown of the given ship
    pub async fn get_extract_cooldown(
        &self,
        ship_id: String,
    ) -> Result<responses::CooldownResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}/extract", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::CooldownResponse>(&response.response_text)
    }

    /// Extract resources near the given ship, optionally targetting specific yields with a survey signature
    pub async fn extract_resources(
        &self,
        ship_id: String,
        survey: Option<shared::Survey>,
    ) -> Result<responses::ExtractResourcesResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        match survey {
            Some(survey) => {
                let extract_request = requests::ExtractRequest { survey };

                response = http_client
                    .execute_request(
                        "POST",
                        &format!("{}/my/ships/{}/extract", &self.base_url, ship_id),
                        Some(&serde_json::to_string(&extract_request).unwrap()),
                        Some(&self.token),
                    )
                    .await?;
            }
            None => {
                response = http_client
                    .execute_request(
                        "POST",
                        &format!("{}/my/ships/{}/extract", &self.base_url, ship_id),
                        None,
                        Some(&self.token),
                    )
                    .await?;
            }
        }

        parse_response::<responses::ExtractResourcesResponse>(&response.response_text)
    }

    /// Dock the ship on the entity it orbits
    pub async fn dock_ship(
        &self,
        ship_id: String,
    ) -> Result<responses::StatusResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/dock", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::StatusResponse>(&response.response_text)
    }

    /// Orbit the ship around the entity it's docked at
    pub async fn orbit_ship(
        &self,
        ship_id: String,
    ) -> Result<responses::StatusResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/orbit", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::StatusResponse>(&response.response_text)
    }

    /// Deliver specified goods for a corresponding contract
    pub async fn deliver_goods(
        &self,
        ship_id: String,
        contract_id: String,
        trade_symbol: String,
        units: u64,
    ) -> Result<responses::DeliveryResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        let contract_delivery_request = requests::ContractDeliveryRequest {
            contract_id,
            trade_symbol,
            units,
        };

        response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/deliver", &self.base_url, ship_id),
                Some(&serde_json::to_string(&contract_delivery_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::DeliveryResponse>(&response.response_text)
    }

    /// Refuel the ship
    pub async fn refuel_ship(
        &self,
        ship_id: String,
    ) -> Result<responses::RefuelResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/refuel", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::RefuelResponse>(&response.response_text)
    }

    /// Scan nearby ships
    pub async fn scan_ships(
        &self,
        ship_id: String,
        scan_mode: shared::ScanMode,
    ) -> Result<responses::ScanResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        let scan_request = requests::ScanShipsRequest { mode: scan_mode };

        response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/scan", &self.base_url, ship_id),
                Some(&serde_json::to_string(&scan_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ScanResponse>(&response.response_text)
    }

    /// Get the scan cooldown of the given ship
    pub async fn get_scan_cooldown(
        &self,
        ship_id: String,
    ) -> Result<responses::CooldownResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}/scan", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::CooldownResponse>(&response.response_text)
    }

    /// Jettison specified cargo
    pub async fn jettison_cargo(
        &self,
        ship_id: String,
        trade_symbol: String,
        units: u64,
    ) -> Result<responses::JettisonResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        let jettison_request = requests::TransactionRequest {
            trade_symbol,
            units,
        };

        response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/jettison", &self.base_url, ship_id),
                Some(&serde_json::to_string(&jettison_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::JettisonResponse>(&response.response_text)
    }

    /// Sell specified cargo at the docked market
    pub async fn sell_cargo(
        &self,
        ship_id: String,
        trade_symbol: String,
        units: u64,
    ) -> Result<responses::TransactionResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        let sell_request = requests::TransactionRequest {
            trade_symbol,
            units,
        };

        response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/sell", &self.base_url, ship_id),
                Some(&serde_json::to_string(&sell_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::TransactionResponse>(&response.response_text)
    }

    /// Buy specified cargo at the docked market
    pub async fn buy_cargo(
        &self,
        ship_id: String,
        trade_symbol: String,
        units: u64,
    ) -> Result<responses::TransactionResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response: SpaceTradersClientResponse;
        let buy_request = requests::TransactionRequest {
            trade_symbol,
            units,
        };

        response = http_client
            .execute_request(
                "POST",
                &format!("{}/my/ships/{}/purchase", &self.base_url, ship_id),
                Some(&serde_json::to_string(&buy_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::TransactionResponse>(&response.response_text)
    }

    // /// Jump specified ship to target destination
    // pub async fn jump(
    //     &self,
    //     ship_id: String,
    //     destination: String,
    // ) -> Result<responses::JumpResponse, SpaceTradersClientError> {
    //     let http_client = self.http_client.lock().await;
    //     let response: SpaceTradersClientResponse;
    //     let jump_request = requests::JumpRequest {
    //         destination
    //     };

    //     response = http_client
    //         .execute_request(
    //             "POST",
    //             &format!("{}/my/ships/{}/jump", &self.base_url, ship_id),
    //             Some(&serde_json::to_string(&jump_request).unwrap()),
    //             Some(&self.token),
    //         )
    //         .await?;

    //     parse_response::<responses::JumpResponse>(&response.response_text)
    // }

    /// Get the jump cooldown of the given ship
    pub async fn get_jump_cooldown(
        &self,
        ship_id: String,
    ) -> Result<responses::CooldownResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/my/ships/{}/jump", &self.base_url, ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::CooldownResponse>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// Systems
    //////////////////////////////////////////////
    /// Get system information
    pub async fn get_system_info(
        &self,
        system_symbol: String,
    ) -> Result<responses::SystemInformationResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/systems/{}", &self.base_url, &system_symbol),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemInformationResponse>(&response.response_text)
    }
    /// Get list of all systems
    pub async fn get_systems(
        &self,
    ) -> Result<responses::SystemsListResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/systems", &self.base_url),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsListResponse>(&response.response_text)
    }
    /// Get list of all system waypoints
    pub async fn get_system_waypoints(
        &self,
        system_symbol: String,
    ) -> Result<responses::SystemsWaypointsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/systems/{}/waypoints", &self.base_url, system_symbol),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsWaypointsResponse>(&response.response_text)
    }
    /// Get system waypoint
    pub async fn get_system_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
    ) -> Result<responses::SystemsWaypointResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "{}/systems/{}/waypoints/{}",
                    &self.base_url, &system_symbol, &waypoint_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsWaypointResponse>(&response.response_text)
    }
    /// Get list of all system shipyards
    pub async fn get_system_shipyards(
        &self,
        system_symbol: String,
    ) -> Result<responses::SystemsShipyardsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/systems/{}/shipyards", &self.base_url, system_symbol),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsShipyardsResponse>(&response.response_text)
    }
    /// Get system shipyard
    pub async fn get_system_shipyard(
        &self,
        system_symbol: String,
        shipyard_symbol: String,
    ) -> Result<responses::SystemsShipyardResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "{}/systems/{}/shipyards/{}",
                    &self.base_url, &system_symbol, &shipyard_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsShipyardResponse>(&response.response_text)
    }
    /// Get shipyard ships
    pub async fn get_shipyard_ships(
        &self,
        system_symbol: String,
        shipyard_symbol: String,
    ) -> Result<responses::ShipyardShipsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "{}/systems/{}/shipyards/{}/ships",
                    &self.base_url, &system_symbol, &shipyard_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::ShipyardShipsResponse>(&response.response_text)
    }
    /// Get list of all system markets
    pub async fn get_system_markets(
        &self,
        system_symbol: String,
    ) -> Result<responses::SystemsMarketsResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("{}/systems/{}/markets", &self.base_url, system_symbol),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsMarketsResponse>(&response.response_text)
    }
    /// Get system market
    pub async fn get_system_market(
        &self,
        system_symbol: String,
        market_symbol: String,
    ) -> Result<responses::SystemsMarketResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "{}/systems/{}/markets/{}",
                    &self.base_url, &system_symbol, &market_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsMarketResponse>(&response.response_text)
    }
}
