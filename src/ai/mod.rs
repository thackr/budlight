pub mod asst;
pub mod msg;

use crate::utils::files::get_glob_set;
use crate::Result;
use async_openai::config::OpenAIConfig;
use async_openai::Client;

// Must provide OpenAI key for GPT4 to work
const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OaClient = Client<OpenAIConfig>;
