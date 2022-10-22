use bytes::Bytes;
use std::net::IpAddr;

const DEFAULT_URL_ENDPOINT: &str = "https://ipinfo.io/ip";

pub async fn new_ip_address(url_endpoint: Option<String>) -> Result<IpAddr, &'static str> {
    let closure_for_default_endpoint = |bytes: Bytes| {
        let parsed_ip_address = String::from_utf8(bytes.to_vec())
            .expect("Response bytes not understandable.")
            .parse()
            .expect("Failed to parse the response into an IP address");
        println!("Parsed IP address: {parsed_ip_address}");
        Ok(parsed_ip_address)
    };
    new_ip_address_with_custom_handler(url_endpoint, closure_for_default_endpoint).await
}

pub async fn new_ip_address_with_custom_handler<F>(
    url_endpoint: Option<String>,
    closure: F,
) -> Result<IpAddr, &'static str>
where
    F: FnOnce(Bytes) -> Result<IpAddr, &'static str>,
{
    let url_endpoint = url_endpoint.unwrap_or_else(|| DEFAULT_URL_ENDPOINT.to_owned());

    println!("Fetching new IP information from \"{url_endpoint}\"");

    let resp = reqwest::get(url_endpoint)
        .await
        .map_err(|_| "Something wrong in the request")?;

    println!("Response body received: {:#?}", resp);

    let resp_bytes = resp
        .bytes()
        .await
        .map_err(|_| "Something wrong in the request bytes")?;

    closure(resp_bytes)
}