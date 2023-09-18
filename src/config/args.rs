use async_trait::async_trait;

use clap::Parser;

use super::{traits::PartialConfigProvider, PartialConfig};

const TG_TOKEN_ARG_LONG: &str = "tg-token";
const TG_TOKEN_ARG_SHORT: char = 't';
const CHAT_ID_ARG_LONG: &str = "chat-id";
const CHAT_ID_ARG_SHORT: char = 'c';
const CHECK_FILE_PATH_ARG_LONG: &str = "check-file-path";
const CHECK_FILE_PATH_ARG_SHORT: char = 'f';
const URL_ENDPOINT_ARG_LONG: &str = "url-endpoint";
const URL_ENDPOINT_ARG_SHORT: char = 'u';
const DRY_RUN_ARG_LONG: &str = "dry";
const DRY_RUN_ARG_SHORT: char = 'd';
const BIND_ADDRESS_ARG_LONG: &str = "bind-address";
const BIND_ADDRESS_ARG_SHORT: char = 'b';

#[derive(Debug, Parser)]
pub struct ArgsPartialConfigProvider {
    #[clap(long = TG_TOKEN_ARG_LONG, short = TG_TOKEN_ARG_SHORT)]
    pub tg_token: Option<String>,
    #[clap(long = CHAT_ID_ARG_LONG, short = CHAT_ID_ARG_SHORT)]
    pub chat_id: Option<String>,
    #[clap(long = CHECK_FILE_PATH_ARG_LONG, short = CHECK_FILE_PATH_ARG_SHORT)]
    pub check_file_path: Option<String>,
    #[clap(long = URL_ENDPOINT_ARG_LONG, short = URL_ENDPOINT_ARG_SHORT)]
    pub url_endpoint: Option<String>,
    #[clap(long = DRY_RUN_ARG_LONG, short = DRY_RUN_ARG_SHORT)]
    pub dry_run: bool,
    #[clap(long = BIND_ADDRESS_ARG_LONG, short = BIND_ADDRESS_ARG_SHORT)]
    pub bind_address: Option<String>,
}

#[async_trait]
impl PartialConfigProvider for ArgsPartialConfigProvider {
    async fn partial_config(&self) -> PartialConfig {
        let config = PartialConfig {
            tg_token: self.tg_token.to_owned(),
            chat_id: self.chat_id.to_owned(),
            check_file_path: self.check_file_path.to_owned(),
            url_endpoint: self.url_endpoint.to_owned(),
            dry_run: Some(self.dry_run),
            bind_address: self.bind_address.to_owned(),
        };
        log::trace!("Arguments passed from the command line: {:?}", config);
        config
    }
}
