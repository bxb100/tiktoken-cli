use clap::Parser;
use serde::{Deserialize, Serialize};

/// A Simple tikToken wrapper CLI
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    // If user give, will counting max_tokens parameter for a chat completion request
    #[arg(short, long, value_enum)]
    pub role: Option<Role>,
    // The OpenAI model
    #[arg(short, long)]
    pub model: String,
    // The target text
    #[arg(short, long)]
    pub text: String,
}
#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
}

impl From<Role> for String {
    fn from(value: Role) -> Self {
        match value {
            Role::System => "system".to_string(),
            Role::User => "user".to_string(),
            Role::Assistant => "assistant".to_string(),
        }
    }
}