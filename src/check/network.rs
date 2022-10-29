use std::net::Ipv4Addr;

use async_trait::async_trait;

use crate::config::Config;

use super::traits::IpAddressProvider;

const DEFAULT_URL_ENDPOINT: &str = "https://ipinfo.io/ip";

pub struct Ipv4Provider<Endpoint>(Endpoint);

impl From<Config> for Ipv4Provider<String> {
    fn from(config: Config) -> Self {
        Self(
            config
                .url_endpoint
                .unwrap_or_else(|| DEFAULT_URL_ENDPOINT.into()),
        )
    }
}

#[async_trait]
impl IpAddressProvider for Ipv4Provider<String> {
    type IpAddress = Ipv4Addr;

    async fn get_current_ip(&self) -> Self::IpAddress {
        println!("Fetching new IP information from \"{:?}\"", self.0);

        let resp = reqwest::get(&self.0)
            .await
            .expect("Something wrong in the request");

        println!("Response body received: {:#?}", resp);

        let resp_bytes = resp
            .bytes()
            .await
            .expect("Something wrong in the request bytes");

        let parsed_ip_address = String::from_utf8(resp_bytes.to_vec())
            .expect("Response bytes not understandable.")
            .parse()
            .expect("Failed to parse the response into an IP address");

        println!("Parsed IP address: {parsed_ip_address}");
        parsed_ip_address
    }
}
