use std::{fmt::Debug, str::FromStr};

use chrono::{DateTime, Utc};

pub mod network;
pub mod storage;
pub mod traits;

#[derive(Debug)]
pub struct IpFetchAttemptInfo<IpAddress> {
    pub ip_address: IpAddress,
    pub datetime: DateTime<Utc>,
    pub last_delivery_success: bool,
}

impl<IpAddress> IpFetchAttemptInfo<IpAddress>
where
    IpAddress: Send + Sync + FromStr + Debug,
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

        log::info!(
            "Parsed datetime: {:?} - Parsed IP address: {:?} - Parsed success: {:?}",
            datetime,
            ip_address,
            last_delivery_success
        );

        Self {
            ip_address,
            datetime,
            last_delivery_success,
        }
    }
}
