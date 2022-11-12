use async_trait::async_trait;

use clap::Parser;

use self::{
    args::ArgsPartialConfigProvider,
    env::EnvPartialConfigProvider,
    traits::{ConfigProvider, PartialConfigProvider},
};

pub mod args;
pub mod env;
pub mod traits;

#[derive(Clone)]
pub struct PartialConfig {
    pub tg_token: Option<String>,
    pub chat_id: Option<String>,
    pub check_file_path: Option<String>,
    pub url_endpoint: Option<String>,
    pub dry_run: Option<bool>,
}

#[derive(Clone)]
pub struct Config {
    pub tg_token: String,
    pub chat_id: String,
    pub check_file_path: String,
    pub url_endpoint: Option<String>,
    pub dry_run: bool,
}

pub struct ArgsOrEnvConfigProvider;

#[async_trait]
impl ConfigProvider for ArgsOrEnvConfigProvider {
    async fn config(&self) -> Config {
        let args = ArgsPartialConfigProvider::parse().partial_config().await;
        let envs = EnvPartialConfigProvider {}.partial_config().await;

        let tg_token = args
            .tg_token
            .or(envs.tg_token)
            .expect("No TG_TOKEN specified.");
        let chat_id = args
            .chat_id
            .or(envs.chat_id)
            .expect("No CHAT_ID specified.");
        let check_file_path = args
            .check_file_path
            .or(envs.check_file_path)
            .expect("No CHECK_FILE_PATH specified.");
        let url_endpoint = args.url_endpoint.or(envs.url_endpoint);
        let dry_run = args.dry_run.or(envs.dry_run).unwrap_or(false);

        Config {
            tg_token,
            chat_id,
            check_file_path,
            url_endpoint,
            dry_run,
        }
    }
}
