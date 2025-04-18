use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures::StreamExt;
use std::error::Error;

use crate::gemini_types::{GeminiChatCompletionResponseStream, GeminiCreateChatCompletionResponse};
fn get_gemini_client() -> Client<OpenAIConfig> {
    let base_url = "https://generativelanguage.googleapis.com/v1beta/openai";
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);
    Client::with_config(config)
}

pub async fn stream_chat(text: String) -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let request = CreateChatCompletionRequestArgs::default()
        //Usage of gemini model
        .model("gemini-2.0-flash")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(text)
                .build()?
                .into(),
        ])
        .n(1)
        .stream(true)
        .max_tokens(500_u32)
        .build()?;

    let mut stream: GeminiChatCompletionResponseStream =
        client.chat().create_stream_byot(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                print!("{}", c.delta.content.clone().unwrap());
            }),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

pub async fn chat_completion(text: String) -> Result<String, Box<dyn Error>> {
    let client = get_gemini_client();
    let request = CreateChatCompletionRequestArgs::default()
        //Usage of gemini model
        .model("gemini-2.0-flash")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(text)
                .build()?
                .into(),
        ])
        // .max_tokens(40_u32)
        .build()?;

    let response: GeminiCreateChatCompletionResponse = client.chat().create_byot(request).await?;

    response
        .choices
        .first() // Get first choice
        .and_then(|choice| choice.message.content.as_ref()) // Get content if exists
        .map(|content| content.to_owned()) // Convert to owned String
        .ok_or_else(|| "No content in response".into()) // Convert None to error
}
