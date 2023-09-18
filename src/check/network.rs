use std::net::Ipv4Addr;

use async_trait::async_trait;
use reqwest::Client;

use crate::config::Config;

use super::traits::IpAddressProvider;

const DEFAULT_URL_ENDPOINT: &str = "https://ipinfo.io/ip";

pub struct Ipv4Provider<Endpoint, BindAddress>(Endpoint, BindAddress);

impl From<Config> for Ipv4Provider<String, Option<Ipv4Addr>> {
    fn from(config: Config) -> Self {
        Self(
            config
                .url_endpoint
                .unwrap_or_else(|| DEFAULT_URL_ENDPOINT.into()),
            config.bind_address.map(|a| {
                a.parse()
                    .expect("The provided bind address is not a valid IPv4 address")
            }),
        )
    }
}

#[async_trait]
impl IpAddressProvider<Ipv4Addr> for Ipv4Provider<String, Option<Ipv4Addr>> {
    async fn get_current_ip(&self) -> Ipv4Addr {
        log::info!("Fetching new IP information from {:?}", self.0);

        let client = Client::builder()
            .local_address(self.1.map(|add| add.into()))
            .build()
            .expect("Failed to create the reqwest client")
            .get(&self.0);
        let resp = client.send().await.expect("Something wrong in the request");

        log::debug!("Response body received: {:#?}", resp);

        let resp_bytes = resp
            .bytes()
            .await
            .expect("Something wrong in the request bytes");

        let parsed_ip_address = String::from_utf8(resp_bytes.to_vec())
            .expect("Response bytes not understandable.")
            .parse()
            .expect("Failed to parse the response into an IP address");

        log::info!("Parsed IP address: {:?}", parsed_ip_address);
        parsed_ip_address
    }
}
