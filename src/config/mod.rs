pub mod args;
pub mod env;

use clap::Parser;

#[derive(Clone)]
pub struct Config {
    pub(super) tg_token: String,
    pub(super) chat_id: String,
    pub(super) check_file_path: String,
    pub(super) url_endpoint: Option<String>,
}

impl Config {
    pub fn parse() -> Self {
        let args_config = args::Config::parse();
        if let (Some(tg_token), Some(chat_id), Some(check_file_path)) = (
            &args_config.tg_token,
            &args_config.chat_id,
            &args_config.check_file_path,
        ) {
            return Self {
                tg_token: tg_token.to_owned(),
                chat_id: chat_id.to_owned(),
                check_file_path: check_file_path.to_owned(),
                url_endpoint: args_config.url_endpoint.to_owned(),
            };
        }
        let env_config = env::Config::parse();
        let tg_token = args_config
            .tg_token
            .or(env_config.tg_token)
            .expect("No TG_TOKEN specified.");
        let chat_id = args_config
            .chat_id
            .or(env_config.chat_id)
            .expect("No CHAT_ID specified.");
        let check_file_path = args_config
            .check_file_path
            .or(env_config.check_file_path)
            .expect("No CHECK_FILE_PATH specified.");
        let url_endpoint = args_config.url_endpoint.or(env_config.url_endpoint);

        Self {
            tg_token,
            chat_id,
            check_file_path,
            url_endpoint,
        }
    }
}
