use anyhow::anyhow;
use clap::Parser;
use tiktoken_rs::{get_bpe_from_model, num_tokens_from_messages, ChatCompletionRequestMessage};

use crate::args::Args;

mod args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let model = args.model;
    let text = args.text;

    if let Some(role) = args.role {
        let messages = vec![ChatCompletionRequestMessage {
            content: Some(text),
            role: role.into(),
            name: None,
            function_call: None,
        }];

        let count = num_tokens_from_messages(&model, &messages).map_err(|e| anyhow!(e))?;
        println!("{count}");
    } else {
        let bpe =  get_bpe_from_model(&model).map_err(|e| anyhow!(e))?;
        let tokens = bpe.encode_with_special_tokens(&text);
        println!("{}", tokens.len());
    }

    Ok(())
}
