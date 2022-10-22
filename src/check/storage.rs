use chrono::prelude::*;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::IpAddr,
    path::Path,
    str::FromStr,
};

pub fn old_ip_address(check_file_path: &Path) -> Option<IpDetails> {
    let file = fs::File::open(check_file_path);
    if file.is_err() {
        // TODO: handle other possible errors
        return None;
    }

    let reader_iter = BufReader::new(file.unwrap()).lines();

    Some(IpDetails::parse(reader_iter))
}

pub fn write_new_ip_address(new_details: IpDetails, check_file_path: &Path) {
    let datetime_line = new_details.datetime().to_rfc2822();
    let ip_line = new_details.ip_address().to_string();

    // Create parent directories if not existing.
    if let Some(parent_path) = check_file_path.parent() {
        fs::create_dir_all(parent_path).expect("Failed to initialize parent directories.");
    }

    // Open file for overwrite
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(check_file_path)
        .expect("Error when opening the check file to update the information.");
    println!("AAA");

    writeln!(file, "{}", datetime_line).expect("Failed to write datetime info to check file.");
    writeln!(file, "{}", ip_line).expect("Failed to write IP info to check file.");
}

// Supposed to be called after write_new_ip_address
pub fn mark_notification_delivery(check_file_path: &Path) {
    // Open file for append
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(false)
        .append(true)
        .open(check_file_path)
        .expect("Error when opening the check file to update the alert information.");
    writeln!(file, "1").expect("Failed to write alert information to check file.");
}

pub struct IpDetails {
    ip_address: IpAddr,
    datetime: DateTime<Utc>,
    last_delivery_success: bool,
}

impl IpDetails {
    pub fn ip_address(&self) -> &IpAddr {
        &self.ip_address
    }

    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }

    pub fn last_delivery_success(&self) -> &bool {
        &self.last_delivery_success
    }
}

impl IpDetails {
    pub fn new(ip_address: IpAddr, datetime: DateTime<Utc>) -> Self {
        Self {
            ip_address,
            datetime,
            last_delivery_success: false,
        }
    }

    // Expected format:
    // Line 1: timestamp
    // Line 2: IP address fetched
    fn parse(mut details: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
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

        let ip_address =
            IpAddr::from_str(&ip_address_raw).expect("Check file corrupt. Invalid IP format.");

        let last_delivery_success = details.next().is_some();

        println!("Parsed datetime: {datetime} - Parsed IP address: {ip_address} - Succeeded: {last_delivery_success}");

        Self {
            ip_address,
            datetime,
            last_delivery_success,
        }
    }
}
