use std::{fmt::Display, str::FromStr};

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait Checker<Config, Payload> {
    async fn should_notify(&self) -> bool;
    async fn get_notification_payload(&self) -> Payload;
}

#[async_trait]
pub(crate) trait IpAddressProvider {
    type IpAddress;

    async fn get_current_ip(&self) -> Self::IpAddress;
}

pub struct IpFetchAttemptInfo<IpAddress: Send + Sync> {
    pub ip_address: IpAddress,
    pub datetime: DateTime<Utc>,
    pub last_delivery_success: bool,
}

impl<IpAddress> IpFetchAttemptInfo<IpAddress>
where
    IpAddress: Send + Sync + FromStr + Display,
{
    pub fn parse(mut details: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
        let datetime_raw = details
            .next()
            .expect("Failed to fetch 1st check file line.")
            .expect("Check file corrupt. No timestamp could be found.");

        let offset_datetime = DateTime::parse_from_rfc2822(&datetime_raw)
            .expect("Check file corrupt. Invalid datetime format.");
        let datetime = DateTime::<Utc>::from(offset_datetime);

        let ip_address_raw = details
            .next()
            .expect("Failed to fetch 2nd check file line.")
            .expect("Check file corrupt. No timestamp could be found.");

        let ip_address = IpAddress::from_str(&ip_address_raw)
            .map_err(|_| "")
            .expect("Check file corrupt. Invalid IP format.");

        let last_delivery_success = details.next().is_some();

        println!("Parsed datetime: {datetime} - Parsed IP address: {ip_address} - Succeeded: {last_delivery_success}");

        Self {
            ip_address,
            datetime,
            last_delivery_success,
        }
    }
}

#[async_trait]
pub(crate) trait UpdatesStorage {
    type IpAddress: Send + Sync;

    async fn get_last_ip_attempt(&self) -> Option<IpFetchAttemptInfo<Self::IpAddress>>;
    async fn save_new_attempt(&self, new_attempt: IpFetchAttemptInfo<Self::IpAddress>);
}
