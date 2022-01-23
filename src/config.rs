use clap::StructOpt;

pub mod env {
    const TG_TOKEN_KEY: &str = "TG_TOKEN";
    const CHAT_ID_KEY: &str = "CHAT_ID";
    const CHECK_FILE_PATH_KEY: &str = "CHECK_FILE_PATH";
    const URL_ENDPOINT_KEY: &str = "URL_ENDPOINT";

    #[derive(Debug)]
    pub(super) struct Config {
        pub(super) tg_token: Option<String>,
        pub(super) chat_id: Option<String>,
        pub(super) check_file_path: Option<String>,
        pub(super) url_endpoint: Option<String>,
    }

    impl Config {
        pub(super) fn parse() -> Self {
            let tg_token = std::env::var(TG_TOKEN_KEY).ok();
            let chat_id = std::env::var(CHAT_ID_KEY).ok();
            let check_file_path = std::env::var(CHECK_FILE_PATH_KEY).ok();
            let url_endpoint = std::env::var(URL_ENDPOINT_KEY).ok();
            Self {
                tg_token,
                chat_id,
                check_file_path,
                url_endpoint,
            }
        }
    }
}

pub mod args {
    use clap::Parser;

    const TG_TOKEN_ARG_LONG: &str = "tg-token";
    const TG_TOKEN_ARG_SHORT: char = 't';
    const CHAT_ID_ARG_LONG: &str = "chat-id";
    const CHAT_ID_ARG_SHORT: char = 'c';
    const CHECK_FILE_PATH_ARG_LONG: &str = "check-file-path";
    const CHECK_FILE_PATH_ARG_SHORT: char = 'f';
    const URL_ENDPOINT_ARG_LONG: &str = "url-endpoint";
    const URL_ENDPOINT_ARG_SHORT: char = 'u';

    #[derive(Debug, Parser)]
    pub(super) struct Config {
        #[clap(long = TG_TOKEN_ARG_LONG, short = TG_TOKEN_ARG_SHORT)]
        pub(super) tg_token: Option<String>,
        #[clap(long = CHAT_ID_ARG_LONG, short = CHAT_ID_ARG_SHORT)]
        pub(super) chat_id: Option<String>,
        #[clap(long = CHECK_FILE_PATH_ARG_LONG, short = CHECK_FILE_PATH_ARG_SHORT)]
        pub(super) check_file_path: Option<String>,
        #[clap(long = URL_ENDPOINT_ARG_LONG, short = URL_ENDPOINT_ARG_SHORT)]
        pub(super) url_endpoint: Option<String>,
    }
}

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
