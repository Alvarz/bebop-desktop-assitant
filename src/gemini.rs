use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
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
    // let api_key = "AIzaSyABKXfSIL-bD5N7vayzu-QcpcMj6Mk7CEA";
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
        .messages(vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                    text.into(),
                ),
                ..Default::default()
            },
        )])
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
        .messages([ChatCompletionRequestMessage::User(
            //"How old is the human civilization?".into(),
            text.into(),
        )])
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
