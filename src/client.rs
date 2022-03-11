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
            &"https://v2-0-0.alpha.spacetraders.io/agents",
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
    pub fn new(http_client: HttpClient, agentname: String, token: String) -> Client {
        Client {
            http_client,
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
                "https://api.spacetraders.io/my/agent",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::AgentDetails>(&response.response_text)
    }

    // //////////////////////////////////////////////
    // ///// ACCOUNT
    // //////////////////////////////////////////////

    // /// Get all information about the current user
    // pub async fn get_my_info(&self) -> Result<responses::UserInfo, SpaceTradersClientError> {
    //     let http_client = self.http_client.lock().await;
    //     let response = http_client
    //         .execute_request(
    //             "GET",
    //             "https://api.spacetraders.io/my/account",
    //             None,
    //             Some(&self.token),
    //         )
    //         .await?;

    //     parse_response::<responses::UserInfo>(&response.response_text)
    // }

    // //////////////////////////////////////////////
    // ///// FLIGHT PLANS
    // //////////////////////////////////////////////

    // /// Get the current details of a flight plan
    // ///
    // /// # Arguments
    // ///
    // /// * `flight_plan_id` - A string containing the flight plan id
    // pub async fn get_flight_plan(
    //     &self,
    //     flight_plan_id: String,
    // ) -> Result<responses::FlightPlan, SpaceTradersClientError> {
    //     let http_client = self.http_client.lock().await;
    //     let response = http_client
    //         .execute_request(
    //             "GET",
    //             &format!(
    //                 "https://api.spacetraders.io/my/flight-plans/{}",
    //                 flight_plan_id
    //             ),
    //             None,
    //             Some(&self.token),
    //         )
    //         .await?;

    //     parse_response::<responses::FlightPlan>(&response.response_text)
    // }

    // /// Create a flight plan.
    // ///
    // /// # Arguments
    // ///
    // /// * `ship_id` - A string containing the ship_id to create the flight plan for
    // /// * `destination` - A string containing the location to send the ship to
    // pub async fn create_flight_plan(
    //     &self,
    //     ship_id: String,
    //     destination: String,
    // ) -> Result<responses::FlightPlan, SpaceTradersClientError> {
    //     let flight_plan_request = requests::FlightPlanRequest {
    //         ship_id: ship_id.clone(),
    //         destination: destination.clone(),
    //     };

    //     let http_client = self.http_client.lock().await;
    //     let response = http_client
    //         .execute_request(
    //             "POST",
    //             "https://api.spacetraders.io/my/flight-plans",
    //             Some(&serde_json::to_string(&flight_plan_request).unwrap()),
    //             Some(&self.token),
    //         )
    //         .await?;

    //     parse_response::<responses::FlightPlan>(&response.response_text)
    // }
}
