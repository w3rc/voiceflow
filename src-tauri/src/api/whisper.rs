use reqwest::multipart;
use serde::Deserialize;

#[derive(Deserialize)]
struct WhisperResponse {
    text: String,
}

pub async fn transcribe(
    api_key: &str,
    wav_data: Vec<u8>,
    prompt: Option<&str>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let file_part = multipart::Part::bytes(wav_data)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| format!("Failed to create multipart: {}", e))?;

    let mut form = multipart::Form::new()
        .part("file", file_part)
        .text("model", "whisper-1")
        .text("response_format", "json");

    if let Some(p) = prompt {
        form = form.text("prompt", p.to_string());
    }

    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Whisper API request failed: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Whisper API error ({}): {}", status, body));
    }

    let result: WhisperResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Whisper response: {}", e))?;

    Ok(result.text)
}
