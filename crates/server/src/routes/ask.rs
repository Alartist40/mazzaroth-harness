use crate::state::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub question: String,
}

#[derive(Debug, Serialize)]
pub struct Citation {
    pub doc_id: String,
    pub title: String,
    pub title_path: String,
    pub license: String,
    pub retrieved_date: String,
    pub snippet: String,
}

pub async fn handle_ask(
    State(state): State<ServerState>,
    Json(req): Json<AskRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let q = req.question.trim().to_string();
    if q.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 1. Retrieve top chunks from SQLite FTS5
    let top_k = state.config.profile.top_k();
    let hits = state.db.search(&q, top_k).unwrap_or_default();

    let citations: Vec<Citation> = hits
        .iter()
        .map(|h| Citation {
            doc_id: h.doc_id.clone(),
            title: h.doc_title.clone(),
            title_path: h.title_path.clone(),
            license: h.license.clone(),
            retrieved_date: h.retrieved_date.clone(),
            snippet: h.snippet.clone(),
        })
        .collect();

    let is_reachable = state.is_llm_reachable().await;
    let endpoint = state.config.llm_endpoint.clone();
    let model = state.config.llm_model.clone();

    let stream = async_stream::stream! {
        // Check if no passages found
        if hits.is_empty() {
            yield Ok(Event::default().event("token").data("I don't have that in the library."));
            yield Ok(Event::default().event("done").data("[DONE]"));
            return;
        }

        // Emit citation block only when citations exist
        if !citations.is_empty() {
            if let Ok(citations_json) = serde_json::to_string(&citations) {
                yield Ok(Event::default().event("citations").data(citations_json));
            }
        }

        if !is_reachable {
            yield Ok(Event::default().event("token").data(
                "Librarian AI is offline. However, the relevant passages have been retrieved above from the library."
            ));
            yield Ok(Event::default().event("done").data("[DONE]"));
            return;
        }

        // Assemble grounded RAG context with budget constraints
        let max_ctx = state.config.profile.max_context_tokens().max(512);
        let max_chars = max_ctx * 3; // Approx 3 chars per token budget for prompt context
        let mut context_text = String::new();
        for (i, h) in hits.iter().enumerate() {
            let passage = format!(
                "[{}] ({}, Date: {}, License: {}) {}:\n\"{}\"\n\n",
                i + 1,
                h.doc_title,
                h.retrieved_date,
                h.license,
                h.title_path,
                h.text
            );
            if context_text.len() + passage.len() > max_chars && !context_text.is_empty() {
                break;
            }
            context_text.push_str(&passage);
        }

        let system_prompt = "You are the Librarian of an offline collection of books. Answer ONLY from the provided passages. If the passages do not contain the answer, say \"I don't have that in the library.\" Quote precisely. Never invent procedures or dosages.";
        let user_prompt = format!("Question: {}\n\nContext Passages:\n{}", q, context_text);

        // Client with connection timeout (10s), without global response stream cutoff
        let client = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
            
        let ollama_url = format!("{}/api/generate", endpoint.trim_end_matches('/'));
        let req_body = serde_json::json!({
            "model": model,
            "prompt": format!("System: {}\n\nUser: {}", system_prompt, user_prompt),
            "stream": true,
            "options": {
                "temperature": 0.2,
                "num_ctx": max_ctx
            }
        });

        match client.post(&ollama_url).json(&req_body).send().await {
            Ok(resp) if resp.status().is_success() => {
                let mut byte_stream = resp.bytes_stream();
                let mut buffer = String::new();
                
                // Allow slow CPU / SBC inference with a generous 45s idle chunk timeout
                while let Ok(Some(item)) = tokio::time::timeout(std::time::Duration::from_secs(45), byte_stream.next()).await {
                    if let Ok(bytes) = item {
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            buffer.push_str(text);
                            while let Some(pos) = buffer.find('\n') {
                                let line = buffer[..pos].trim().to_string();
                                buffer = buffer[pos + 1..].to_string();
                                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                                    if let Some(token) = val["response"].as_str() {
                                        yield Ok(Event::default().event("token").data(token));
                                    }
                                    if val["done"].as_bool() == Some(true) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // Fallback / standard response
                yield Ok(Event::default().event("token").data(
                    "Retrieved relevant passages from the library above."
                ));
            }
        }

        yield Ok(Event::default().event("done").data("[DONE]"));
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
