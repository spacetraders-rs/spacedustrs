use spacedust;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TestClient {
    pub http_client: Arc<Mutex<spacedust::client::SpaceTradersClient>>,
    pub client: spacedust::client::Client,
}
impl TestClient {
    pub fn new() -> Self {
        Self {
            http_client: spacedust::client::get_http_client(None),
            client: spacedust::client::Client::new(
                spacedust::client::get_http_client(None),
                "https://v2-0-0.alpha.spacetraders.io".to_string(),
                "GRNTST69".to_string(),
                "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiR1JOVFNUNjkiLCJpYXQiOjE2NDcwOTg4MzgsInN1YiI6ImFnZW50LXRva2VuIn0.Jrvr58MMHGf2D4_gjiUr3ctdfSmdCIKGDivm80NMyEZ0QdYdE5K0mhhAyUfIKGKpAHfMoroCclnkFkk31b9XexWFrRM-denYVIj4-p_-NPAeP3A6QFLryu4UO4RR4ItPL0n5Njf8ZZ3LxcF3tQir2kSMHYLGKEFNTNs78PWzX8uhn5ottgi90d5HaAK3D3nDuRs31oNwv6uzGDwrhQvHa2jQ0QunNbPG9BZIMW7uij60HAxj-_zAm9LxIbHwTYQWG2cGuiCaRXOph33pZ6jVsdRuaQlf-QD1j7s8A-MpzhakGI-m6AeFyxW49MpQDrLsZUXRAibSHLksUlKL2sf-yyZV9kjErmqV9v6TGhhM-izXGVPJ-CrW0To_Qm3WzLQmGQqUC3SPIylTR2LJcAE5D8UAN_18yKUiLffyAw45-OqcUXH5QMDCRV7hGdaIyYmyyYNNIIGABXrvusy5RxTgCu-utaPa0DmPtEF8xtr2GLGkR_NSwXvg8ztnM1IBZNSu".to_string(),
            )
        }
    }
}
