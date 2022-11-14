use async_trait::async_trait;

use super::{traits::PartialConfigProvider, PartialConfig};

const TG_TOKEN_KEY: &str = "TG_TOKEN";
const CHAT_ID_KEY: &str = "CHAT_ID";
const CHECK_FILE_PATH_KEY: &str = "CHECK_FILE_PATH";
const URL_ENDPOINT_KEY: &str = "URL_ENDPOINT";

pub struct EnvPartialConfigProvider;

#[async_trait]
impl PartialConfigProvider for EnvPartialConfigProvider {
    async fn partial_config(&self) -> PartialConfig {
        let tg_token = std::env::var(TG_TOKEN_KEY).ok();
        let chat_id = std::env::var(CHAT_ID_KEY).ok();
        let check_file_path = std::env::var(CHECK_FILE_PATH_KEY).ok();
        let url_endpoint = std::env::var(URL_ENDPOINT_KEY).ok();

        let config = PartialConfig {
            tg_token,
            chat_id,
            check_file_path,
            url_endpoint,
            // Dry run can only be passed as a CLI argument, not as an env
            dry_run: None,
        };
        log::trace!("Arguments passed from the environment: {:?}", config);
        config
    }
}
