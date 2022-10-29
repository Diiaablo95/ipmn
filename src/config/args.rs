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
pub struct Config {
    #[clap(long = TG_TOKEN_ARG_LONG, short = TG_TOKEN_ARG_SHORT)]
    pub(super) tg_token: Option<String>,
    #[clap(long = CHAT_ID_ARG_LONG, short = CHAT_ID_ARG_SHORT)]
    pub(super) chat_id: Option<String>,
    #[clap(long = CHECK_FILE_PATH_ARG_LONG, short = CHECK_FILE_PATH_ARG_SHORT)]
    pub(super) check_file_path: Option<String>,
    #[clap(long = URL_ENDPOINT_ARG_LONG, short = URL_ENDPOINT_ARG_SHORT)]
    pub(super) url_endpoint: Option<String>,
}
