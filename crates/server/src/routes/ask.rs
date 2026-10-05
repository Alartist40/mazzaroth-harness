use crate::state::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub question: String,
    pub history: Option<Vec<ChatMessage>>,
    pub model: Option<String>,
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

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaTagItem>>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagItem {
    name: String,
}

fn get_library_catalog_summary() -> &'static str {
    "MAZZAROTH OFFLINE KNOWLEDGE DAEMON & CATALOG MANIFEST:
- SCRIPTURE DATABASE: 66 Languages, 226 Translations across the globe.
  Language codes and names present in offline vault:
  AFR (Afrikaans), ALB (Albanian), ARM (Armenian), ARN (Mapudungun), BEN (Bengali), CEB (Cebuano), CES (Czech), CHE (Chechen), CHU (Old Church Slavonic), COP (Coptic), DAN (Danish), DEU (German: 14 versions), ELL (Greek: 3 versions), ENG (English: 38 versions including KJV, ASV, BSB, Geneva 1599, Darby, Tyndale, Wycliffe, Webster, Young's Literal), EPO (Esperanto), EST (Estonian), FIN (Finnish), FRA (French: 5 versions), GLV (Manx), GOT (Gothic), GUJ (Gujarati), HAT (Haitian Creole), HEB (Hebrew: 4 versions), HIN (Hindi), HRV (Croatian), HUN (Hungarian), IND (Indonesian), ITA (Italian: 3 versions), JPN (Japanese: 3 versions), KAN (Kannada), KOR (Korean: 2 versions), LAT (Latin: 3 versions), MAL (Malayalam), MAR (Marathi), MLG (Malagasy), MRI (Maori), MYA (Burmese), NEP (Nepali), NLD (Dutch: 8 versions), NOR (Norwegian), NSO (Northern Sotho), ORI (Odia), PAN (Punjabi), POL (Polish: 2 versions), PON (Pohnpeian), POR (Portuguese: 3 versions), RUS (Russian: 3 versions), SAM (Samaritan), SLV (Slovenian), SML (Central Sama), SPA (Spanish: 4 versions), SRP (Serbian: 2 versions), SWE (Swedish: 3 versions), SYR (Syriac), TAM (Tamil), TEL (Telugu: 2 versions), TGL (Tagalog: 2 versions), THA (Thai), TPI (Tok Pisin), TSG (Tausug), UKR (Ukrainian), VIE (Vietnamese), VLS (West Flemish), XHO (Xhosa), ZHO (Chinese: 4 versions), ZUL (Zulu).
  [IMPORTANT INVENTORY NOTE: Languages NOT listed above, such as Twi (Ghana), Yoruba, Swahili, Amharic, Farsi, etc., are currently NOT in this offline database].
- ASTRONOMY & CELESTIAL ASTROMETRY: Star Navigation Handbook, Polaris Alignment, Vega ground truth, 88 standard constellations, 12 Zodiac asterisms, closed-form Alt/Az dome astrometry engine.
- SURVIVAL & WILDERNESS EXPEDITION: FM 21-76 Survival Manual, water purification, shelter construction, navigation corridors, triage.
- EMERGENCY MEDICAL: Emergency Medical Protocols, triage, airway intervention, wound management.
- COGNITIVE MEMORY: Neural memory stars, semantic nexus linkages, and local personal knowledge graph.
- CARTOGRAPHY: Sovereign offline MapLibre vector maps and PMTiles regional street packs."
}

async fn resolve_ollama_model(client: &reqwest::Client, endpoint: &str, requested_model: &str) -> String {
    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
    if let Ok(resp) = client.get(&url).send().await {
        if resp.status().is_success() {
            if let Ok(tags) = resp.json::<OllamaTagsResponse>().await {
                if let Some(models) = tags.models {
                    let available_names: Vec<String> = models.into_iter().map(|m| m.name).collect();
                    // 1. Exact match
                    if available_names.iter().any(|n| n == requested_model) {
                        return requested_model.to_string();
                    }
                    // 2. Fuzzy / clean prefix match
                    let req_clean = requested_model.replace([':', '-', '.'], "").to_lowercase();
                    for name in &available_names {
                        let name_clean = name.replace([':', '-', '.'], "").to_lowercase();
                        if name_clean.starts_with(&req_clean) || req_clean.starts_with(&name_clean) {
                            return name.clone();
                        }
                    }
                    // 3. First non-embed completion model
                    for name in &available_names {
                        if !name.contains("embed") {
                            return name.clone();
                        }
                    }
                }
            }
        }
    }
    requested_model.to_string()
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
    let requested_model = req
        .model
        .as_ref()
        .filter(|m| !m.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| state.config.llm_model.clone());
    let history = req.history.unwrap_or_default();

    let stream = async_stream::stream! {
        // If LLM is offline and no passages found -> return standard refusal
        if !is_reachable && hits.is_empty() {
            if let Ok(tok) = serde_json::to_string("I don't have that in the library.") {
                yield Ok(Event::default().event("token").data(tok));
            }
            yield Ok(Event::default().event("done").data("[DONE]"));
            return;
        }

        // Emit citation block when citations exist
        if !citations.is_empty() {
            if let Ok(citations_json) = serde_json::to_string(&citations) {
                yield Ok(Event::default().event("citations").data(citations_json));
            }
        }

        if !is_reachable {
            if let Ok(tok) = serde_json::to_string(
                "Librarian AI is offline. However, the relevant passages have been retrieved above from the library."
            ) {
                yield Ok(Event::default().event("token").data(tok));
            }
            yield Ok(Event::default().event("done").data("[DONE]"));
            return;
        }

        // Client with connection timeout (10s)
        let client = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        let resolved_model = resolve_ollama_model(&client, &endpoint, &requested_model).await;

        // Assemble grounded RAG context with budget constraints
        let max_ctx = state.config.profile.max_context_tokens().max(2048);
        let max_chars = max_ctx * 4;
        let mut context_text = format!("=== LIBRARY CATALOG MANIFEST ===\n{}\n\n", get_library_catalog_summary());
        if !hits.is_empty() {
            context_text.push_str("=== RETRIEVED DOCUMENT PASSAGES ===\n");
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
                if context_text.len() + passage.len() > max_chars {
                    break;
                }
                context_text.push_str(&passage);
            }
        }

        let system_prompt = format!(
            "You are Mazzaroth, an intelligent, eloquent, and sovereign AI Librarian and polyglot scholar. You reside within a private offline cognitive memory vault.\n\n\
Your Conversational Purpose:\n\
- Engage in rich, natural, and insightful conversations with the user. You are a conversational scholar companion, not a mechanical search filter.\n\
- Discuss literature, theology, history, astronomy, and science with depth, intellectual warmth, and nuance.\n\
- When discussing scriptures or world texts, feel free to compare verses across translations, analyze original linguistic meanings (e.g. Greek, Hebrew, Latin, English, German, French, etc.), and provide rich context.\n\
- When relevant document passages are provided in your context, weave them naturally into your response and cite them.\n\
- For catalog and database inventory inquiries (such as what languages or books are in the vault), speak authoritatively using the Library Catalog Manifest below.\n\
- If the question is completely unrelated to anything in the library and no text exists, politely let the user know: \"I don't have that in the library.\"\n\n\
{}",
            context_text
        );

        // Build multi-turn chat messages
        let mut messages = Vec::new();
        messages.push(serde_json::json!({
            "role": "system",
            "content": system_prompt
        }));

        // Append recent conversation history (up to last 8 turns)
        let hist_start = if history.len() > 8 { history.len() - 8 } else { 0 };
        for msg in &history[hist_start..] {
            if msg.role == "user" || msg.role == "assistant" {
                messages.push(serde_json::json!({
                    "role": msg.role,
                    "content": msg.content
                }));
            }
        }

        // Current user message
        messages.push(serde_json::json!({
            "role": "user",
            "content": q
        }));

        let ollama_chat_url = format!("{}/api/chat", endpoint.trim_end_matches('/'));
        let req_body = serde_json::json!({
            "model": resolved_model,
            "messages": messages,
            "stream": true,
            "options": {
                "temperature": 0.6,
                "num_ctx": max_ctx
            }
        });

        match client.post(&ollama_chat_url).json(&req_body).send().await {
            Ok(resp) if resp.status().is_success() => {
                let mut byte_stream = resp.bytes_stream();
                let mut buffer = String::new();
                let mut is_completed = false;

                loop {
                    match tokio::time::timeout(std::time::Duration::from_secs(45), byte_stream.next()).await {
                        Ok(Some(Ok(bytes))) => {
                            if let Ok(text) = std::str::from_utf8(&bytes) {
                                buffer.push_str(text);
                                while let Some(pos) = buffer.find('\n') {
                                    let line = buffer[..pos].trim().to_string();
                                    buffer = buffer[pos + 1..].to_string();
                                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                                        let token = val["message"]["content"]
                                            .as_str()
                                            .or_else(|| val["response"].as_str());
                                        if let Some(tok) = token {
                                            if !tok.is_empty() {
                                                if let Ok(tok_json) = serde_json::to_string(tok) {
                                                    yield Ok(Event::default().event("token").data(tok_json));
                                                }
                                            }
                                        }
                                        if val["done"].as_bool() == Some(true) {
                                            is_completed = true;
                                            break;
                                        }
                                    }
                                }
                                if is_completed {
                                    break;
                                }
                            }
                        }
                        Ok(Some(Err(_))) => {
                            if let Ok(tok) = serde_json::to_string("\n[stream read error]") {
                                yield Ok(Event::default().event("token").data(tok));
                            }
                            break;
                        }
                        Ok(None) => {
                            break;
                        }
                        Err(_) => {
                            if let Ok(tok) = serde_json::to_string("\n[stream interrupted after 45s idle]") {
                                yield Ok(Event::default().event("token").data(tok));
                            }
                            break;
                        }
                    }
                }
            }
            Ok(resp) => {
                let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                if let Ok(tok) = serde_json::to_string(&format!("[Ollama error: {}]", err_text)) {
                    yield Ok(Event::default().event("token").data(tok));
                }
            }
            Err(e) => {
                if let Ok(tok) = serde_json::to_string(&format!("[Ollama connection error: {}]", e)) {
                    yield Ok(Event::default().event("token").data(tok));
                }
            }
        }

        yield Ok(Event::default().event("done").data("[DONE]"));
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
