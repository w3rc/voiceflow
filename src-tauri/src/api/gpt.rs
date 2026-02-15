use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

pub async fn cleanup_text(
    api_key: &str,
    raw_transcript: &str,
    context: &str,
) -> Result<String, String> {
    let system_prompt = format!(
        r#"You are a dictation assistant. Clean up the following raw speech transcript:
- Fix grammar and punctuation
- Remove filler words (um, uh, like, you know)
- Maintain the speaker's intent and meaning
- Do NOT add information that wasn't spoken
- Match the appropriate tone for the context: {}
- Return ONLY the cleaned text, nothing else"#,
        context
    );

    call_gpt(api_key, &system_prompt, raw_transcript).await
}

pub async fn execute_command(
    api_key: &str,
    selected_text: &str,
    voice_command: &str,
) -> Result<String, String> {
    let system_prompt = r#"You are a text transformation assistant. The user has selected some text and given a voice command describing how to transform it.
Apply the requested transformation to the selected text.
Return ONLY the transformed text, nothing else.
Do not add explanations or commentary."#;

    let user_message = format!(
        "Selected text:\n{}\n\nCommand: {}",
        selected_text, voice_command
    );

    call_gpt(api_key, system_prompt, &user_message).await
}

async fn call_gpt(
    api_key: &str,
    system_prompt: &str,
    user_message: &str,
) -> Result<String, String> {
    let client = Client::new();

    let request = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            },
        ],
        temperature: 0.3,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("GPT API request failed: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("GPT API error ({}): {}", status, body));
    }

    let result: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GPT response: {}", e))?;

    result
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No response from GPT".to_string())
}
