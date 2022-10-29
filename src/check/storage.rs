use std::{
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Write},
    marker::PhantomData,
    path::PathBuf,
    str::FromStr,
};

use async_trait::async_trait;

use crate::config::Config;

use super::traits::{IpFetchAttemptInfo, UpdatesStorage};

pub struct FileSystemUpdatesStorage<IpAddress>(PathBuf, PhantomData<IpAddress>);

impl<IpAddress> From<Config> for FileSystemUpdatesStorage<IpAddress> {
    fn from(config: Config) -> Self {
        Self(config.check_file_path.into(), PhantomData)
    }
}

#[async_trait]
impl<IpAddress> UpdatesStorage for FileSystemUpdatesStorage<IpAddress>
where
    IpAddress: Send + Sync + FromStr + Display,
{
    type IpAddress = IpAddress;

    async fn get_last_ip_attempt(&self) -> Option<IpFetchAttemptInfo<Self::IpAddress>> {
        let file = fs::File::open(&self.0);
        if file.is_err() {
            // TODO: handle other possible errors
            return None;
        }

        let reader_iter = BufReader::new(file.unwrap()).lines();
        Some(IpFetchAttemptInfo::parse(reader_iter))
    }

    async fn save_new_attempt(&self, new_attempt: IpFetchAttemptInfo<Self::IpAddress>) {
        let datetime_line = new_attempt.datetime.to_rfc2822();
        let ip_line = new_attempt.ip_address.to_string();

        // Create parent directories if not existing.
        if let Some(parent_path) = self.0.parent() {
            fs::create_dir_all(parent_path).expect("Failed to initialize parent directories.");
        }

        // Open file for overwrite
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.0)
            .expect("Error when opening the check file to update the information.");

        writeln!(file, "{}", datetime_line).expect("Failed to write datetime info to check file.");
        writeln!(file, "{}", ip_line).expect("Failed to write IP info to check file.");
    }
}
