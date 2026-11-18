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
    pub worker: Option<String>,
    pub lang: Option<String>,
    pub llm_endpoint: Option<String>,
}

fn resolve_llm_endpoint(override_url: Option<&str>, default_url: &str) -> String {
    if let Some(raw) = override_url {
        let trimmed = raw.trim();
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return trimmed.trim_end_matches('/').to_string();
        }
    }
    default_url.to_string()
}

struct WorkerProfile {
    id: &'static str,
    name: &'static str,
    location: &'static str,
    categories: &'static [&'static str],
    persona: &'static str,
}

fn resolve_worker_profile(worker_id: Option<&str>) -> WorkerProfile {
    match worker_id.unwrap_or("coordinator").trim().to_lowercase().as_str() {
        "archivist" => WorkerProfile {
            id: "archivist",
            name: "Archivist",
            location: "bookshelf",
            categories: &["scripture"],
            persona: "You are the Archivist, specialist in ancient scriptures, world translations, and linguistic comparison.",
        },
        "astronomer" => WorkerProfile {
            id: "astronomer",
            name: "Astronomer",
            location: "orrery",
            categories: &["astronomy"],
            persona: "You are the Astronomer, specialist in celestial navigation, constellations, Polaris alignment, and astrometry.",
        },
        "medic" => WorkerProfile {
            id: "medic",
            name: "Field Medic",
            location: "map_table",
            categories: &["medical"],
            persona: "You are the Field Medic, specialist in clinical medicine, anatomy, pathology, and emergency medical protocols.",
        },
        "pathfinder" | "field" => WorkerProfile {
            id: "pathfinder",
            name: "The Pathfinder",
            location: "map_table",
            categories: &["survival", "pathfinders", "youth"],
            persona: "You are the Pathfinder, specialist in wilderness survival (FM 21-76), expedition craft, knotcraft, and cartography.",
        },
        "chronicler" => WorkerProfile {
            id: "chronicler",
            name: "Chronicler",
            location: "terminal",
            categories: &[],
            persona: "You are the Chronicler, specialist in cross-referencing the entire Mazzaroth knowledge vault and personal memory notes.",
        },
        _ => WorkerProfile {
            id: "coordinator",
            name: "Chief Librarian",
            location: "desk",
            categories: &[],
            persona: "You are the Chief Librarian of Mazzaroth, coordinating all vault archives and answering with grounded precision.",
        },
    }
}

fn make_office_event(
    stage: &str,
    worker: &str,
    location: Option<&str>,
    detail: &str,
    extra: serde_json::Value,
) -> Option<Event> {
    let mut payload = serde_json::json!({
        "stage": stage,
        "worker": worker,
        "detail": detail,
    });
    if let Some(loc) = location {
        payload["location"] = serde_json::json!(loc);
    }
    if let Some(obj) = extra.as_object() {
        for (k, v) in obj {
            payload[k] = v.clone();
        }
    }
    serde_json::to_string(&payload)
        .ok()
        .map(|s| Event::default().event("office").data(s))
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

fn is_small_model(model: &str) -> bool {
    let m = model.to_lowercase();
    [":0.5b", ":1b", ":1.5b", ":1.7b", ":2b", ":3b", "1b-", "3b-", "mini", "tiny", "nano", "phi3"]
        .iter()
        .any(|pat| m.contains(pat))
}

#[derive(Debug, Deserialize)]
pub struct WarmRequest {
    pub model: Option<String>,
    pub llm_endpoint: Option<String>,
}

/// Pre-warms the selected Ollama model with a 10m keep_alive so cold-load latency disappears.
pub async fn handle_warm(
    State(state): State<ServerState>,
    Json(req): Json<WarmRequest>,
) -> Json<serde_json::Value> {
    let endpoint = resolve_llm_endpoint(req.llm_endpoint.as_deref(), &state.config.llm_endpoint);
    let requested_model = req
        .model
        .as_ref()
        .filter(|m| !m.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| state.config.llm_model.clone());

    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(3))
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default();

    let resolved_model = resolve_ollama_model(&client, &endpoint, &requested_model).await;
    let url = format!("{}/api/generate", endpoint.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": resolved_model,
        "prompt": "",
        "stream": false,
        "keep_alive": "10m"
    });

    let ok = match client.post(&url).json(&body).send().await {
        Ok(r) => r.status().is_success(),
        Err(_) => false,
    };

    Json(serde_json::json!({
        "ok": ok,
        "model": resolved_model
    }))
}

pub async fn handle_ask(
    State(state): State<ServerState>,
    Json(req): Json<AskRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let q = req.question.trim().to_string();
    if q.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let worker = resolve_worker_profile(req.worker.as_deref());
    let is_ja = req
        .lang
        .as_deref()
        .is_some_and(|l| l.eq_ignore_ascii_case("ja"));
    let pref_lang = if is_ja { Some("ja") } else { None };

    // 1. Retrieve top chunks from SQLite FTS5 (scoped to worker's specialty + language first, fallback to global)
    let top_k = state.config.profile.top_k();
    let search_start = std::time::Instant::now();
    let mut hits = state
        .db
        .search_by_categories_and_lang(&q, worker.categories, pref_lang, top_k)
        .unwrap_or_default();
    let mut scoped_fallback = false;
    if hits.is_empty() && pref_lang.is_some() {
        // Cross-lingual fallback: search all languages in worker's categories
        hits = state
            .db
            .search_by_categories_and_lang(&q, worker.categories, None, top_k)
            .unwrap_or_default();
    }
    if hits.is_empty() && !worker.categories.is_empty() {
        hits = state.db.search(&q, top_k).unwrap_or_default();
        scoped_fallback = true;
    }
    let search_ms = search_start.elapsed().as_millis() as u64;

    let unique_docs: std::collections::HashSet<&str> =
        hits.iter().map(|h| h.doc_id.as_str()).collect();
    let unique_doc_count = unique_docs.len();

    // Detect if Chief Librarian should visibly consult a domain specialist
    let consult_worker: Option<&'static str> = if worker.id == "coordinator" {
        let ql = q.to_lowercase();
        if hits.iter().any(|h| h.category == "scripture") || ql.contains("bible") || ql.contains("verse") || ql.contains("scripture") {
            Some("archivist")
        } else if hits.iter().any(|h| h.category == "astronomy") || ql.contains("star") || ql.contains("polaris") || ql.contains("constellation") {
            Some("astronomer")
        } else if hits.iter().any(|h| h.category == "medical") || ql.contains("medical") || ql.contains("triage") || ql.contains("airway") {
            Some("medic")
        } else if hits.iter().any(|h| h.category == "survival" || h.category == "pathfinders") || ql.contains("survival") || ql.contains("water") {
            Some("pathfinder")
        } else {
            None
        }
    } else {
        None
    };

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

    let endpoint = resolve_llm_endpoint(req.llm_endpoint.as_deref(), &state.config.llm_endpoint);
    let is_reachable = if req.llm_endpoint.is_some() {
        let probe_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap_or_default();
        probe_client
            .get(format!("{}/api/tags", endpoint.trim_end_matches('/')))
            .send()
            .await
            .is_ok_and(|r| r.status().is_success())
    } else {
        state.is_llm_reachable().await
    };
    let requested_model = req
        .model
        .as_ref()
        .filter(|m| !m.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| state.config.llm_model.clone());
    let history = req.history.unwrap_or_default();

    let stream = async_stream::stream! {
        // Stage 1: Dispatch worker
        if let Some(ev) = make_office_event(
            "dispatch",
            worker.id,
            Some("desk"),
            &format!("Assigned to {}", worker.name),
            serde_json::json!({ "worker_name": worker.name }),
        ) {
            yield Ok(ev);
        }

        // Optional Stage 1b: Chief Librarian consults domain specialist
        if let Some(consult_id) = consult_worker {
            if let Some(ev) = make_office_event(
                "handoff",
                worker.id,
                Some("desk"),
                &format!("Consulting @{}...", consult_id),
                serde_json::json!({ "consult": consult_id }),
            ) {
                yield Ok(ev);
            }
        }

        // Stage 2: Search station
        let search_loc = if scoped_fallback { "terminal" } else { worker.location };
        if let Some(ev) = make_office_event(
            "search",
            worker.id,
            Some(search_loc),
            &format!("Scanning FTS5 index ({}ms)...", search_ms),
            serde_json::json!({ "ms": search_ms }),
        ) {
            yield Ok(ev);
        }

        // Stage 3: Found hits
        if let Some(ev) = make_office_event(
            "found",
            worker.id,
            Some(search_loc),
            &format!("Found {} passages across {} docs", hits.len(), unique_doc_count),
            serde_json::json!({ "hits": hits.len(), "docs": unique_doc_count, "ms": search_ms }),
        ) {
            yield Ok(ev);
        }

        // If LLM is offline and no passages found -> return standard refusal
        if !is_reachable && hits.is_empty() {
            if let Ok(tok) = serde_json::to_string("I don't have that in the library.") {
                yield Ok(Event::default().event("token").data(tok));
            }
            if let Some(ev) = make_office_event("done", worker.id, Some("desk"), "Task complete (offline)", serde_json::json!({})) {
                yield Ok(ev);
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
            if let Some(ev) = make_office_event("done", worker.id, Some("desk"), "Passages retrieved (Ollama offline)", serde_json::json!({})) {
                yield Ok(ev);
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
        let slm_mode = is_small_model(&resolved_model);

        // Dynamic Context Budget: SLMs (1B-4B) get compact context & top-4 focused passages
        // so prompt-eval stays fast and the model does not drift or hallucinate.
        let base_ctx = state.config.profile.max_context_tokens().max(2048);
        let effective_ctx = if slm_mode { base_ctx.min(3072) } else { base_ctx };
        let max_chars = if slm_mode { 6000 } else { effective_ctx * 4 };
        let max_passages = if slm_mode { 4 } else { hits.len() };
        let max_snippet_chars = if slm_mode { 800 } else { 4000 };

        let mut context_text = if slm_mode {
            "=== VAULT CATALOG ===\n66-language Scripture DB (226 versions), Astronomy (88 constellations, Polaris), Medical & Survival (FM 21-76), Cognitive Memory & Offline Maps.\n\n".to_string()
        } else {
            format!("=== LIBRARY CATALOG MANIFEST ===\n{}\n\n", get_library_catalog_summary())
        };

        if !hits.is_empty() {
            context_text.push_str("=== RETRIEVED DOCUMENT PASSAGES ===\n");
            for (i, h) in hits.iter().take(max_passages).enumerate() {
                let text_slice: String = h.text.chars().take(max_snippet_chars).collect();
                let passage = format!(
                    "[{}] ({}, Date: {}, License: {}) {}:\n\"{}\"\n\n",
                    i + 1,
                    h.doc_title,
                    h.retrieved_date,
                    h.license,
                    h.title_path,
                    text_slice
                );
                if context_text.len() + passage.len() > max_chars {
                    break;
                }
                context_text.push_str(&passage);
            }
        }

        // Stage 4: Thinking / Prompt Evaluation on local SLM/LLM
        if let Some(ev) = make_office_event(
            "think",
            worker.id,
            Some("desk"),
            &format!(
                "Evaluating {} chars on {}{}",
                context_text.len(),
                resolved_model,
                if slm_mode { " [SLM]" } else { "" }
            ),
            serde_json::json!({
                "model": resolved_model,
                "context_chars": context_text.len(),
                "num_ctx": effective_ctx,
                "slm_mode": slm_mode
            }),
        ) {
            yield Ok(ev);
        }

        let lang_rule = if is_ja {
            "\n- LANGUAGE: Respond in natural, clear Japanese (日本語). Translate English passages accurately if needed while keeping [1], [2] citations."
        } else {
            ""
        };

        let system_prompt = format!(
            "{persona}\n\
You reside in Mazzaroth, a private offline knowledge vault.\n\n\
RULES:\n\
- Be direct, natural, and concise. No AI clichés or filler.\n\
- Lead with verified facts and cite retrieved passages using [1], [2], etc.\n\
- Quote passages verbatim; do not invent quotes or facts outside the passages/catalog.{lang_rule}\n\
- If nothing relevant exists in the vault, say: \"I don't have that in the library.\"\n\n\
{context}",
            persona = worker.persona,
            lang_rule = lang_rule,
            context = context_text
        );

        // Build multi-turn chat messages
        let mut messages = Vec::new();
        messages.push(serde_json::json!({
            "role": "system",
            "content": system_prompt
        }));

        // Append recent conversation history (up to last 6 turns for SLM, 8 for larger)
        let hist_limit = if slm_mode { 6 } else { 8 };
        let hist_start = if history.len() > hist_limit { history.len() - hist_limit } else { 0 };
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
            "think": false,
            "keep_alive": "10m",
            "options": {
                "temperature": if slm_mode { 0.4 } else { 0.6 },
                "num_ctx": effective_ctx
            }
        });

        match client.post(&ollama_chat_url).json(&req_body).send().await {
            Ok(resp) if resp.status().is_success() => {
                let mut byte_stream = resp.bytes_stream();
                let mut raw_byte_buffer = Vec::new();
                let mut full_response_text = String::new();
                let mut is_completed = false;
                let mut streaming_emitted = false;

                loop {
                    match tokio::time::timeout(std::time::Duration::from_secs(45), byte_stream.next()).await {
                        Ok(Some(Ok(bytes))) => {
                            raw_byte_buffer.extend_from_slice(&bytes);
                            while let Some(pos) = raw_byte_buffer.iter().position(|&b| b == b'\n') {
                                let line_bytes = raw_byte_buffer.drain(..=pos).collect::<Vec<u8>>();
                                if let Ok(line_str) = std::str::from_utf8(&line_bytes) {
                                    let line = line_str.trim();
                                    if !line.is_empty() {
                                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                                            let token = val["message"]["content"]
                                                .as_str()
                                                .or_else(|| val["response"].as_str());
                                            if let Some(tok) = token {
                                                if !tok.is_empty() {
                                                    let sanitized_token = librarian_core::QuoteGuard::sanitize_model_token(tok);
                                                    if !sanitized_token.is_empty() {
                                                        if !streaming_emitted {
                                                            streaming_emitted = true;
                                                            if let Some(ev) = make_office_event(
                                                                "stream",
                                                                worker.id,
                                                                Some("desk"),
                                                                "Drafting grounded response...",
                                                                serde_json::json!({}),
                                                            ) {
                                                                yield Ok(ev);
                                                            }
                                                        }
                                                        full_response_text.push_str(&sanitized_token);
                                                        if let Ok(tok_json) = serde_json::to_string(&sanitized_token) {
                                                            yield Ok(Event::default().event("token").data(tok_json));
                                                        }
                                                    }
                                                }
                                            }
                                            if val["done"].as_bool() == Some(true) {
                                                is_completed = true;

                                                // ExactQuoteGuard: Extract model sentence and verify against canonical chunk
                                                let citations_found = librarian_core::QuoteGuard::extract_citations(&full_response_text);
                                                let mut verified_count = 0usize;
                                                for cit in citations_found {
                                                    let surrounding = librarian_core::QuoteGuard::extract_surrounding_sentence(&full_response_text, &cit.raw_citation);

                                                    // Look for best matching hit matching book and chapter/verse
                                                    let matched_hit = hits.iter().find(|h| {
                                                        (h.text.contains(&cit.raw_citation) || h.title_path.contains(&cit.book))
                                                            && (h.text.contains(&format!("{}:{}", cit.chapter, cit.verse))
                                                                || h.text.contains(&format!("{}.", cit.verse))
                                                                || h.text.contains(&format!("[{}]", cit.verse))
                                                                || h.title_path.contains(&format!("Chapter {}", cit.chapter)))
                                                    }).or_else(|| {
                                                        hits.iter().find(|h| h.text.contains(&cit.raw_citation) || h.title_path.contains(&cit.book))
                                                    });

                                                    if let Some(h) = matched_hit {
                                                        let canonical_verse = librarian_core::QuoteGuard::slice_canonical_verse(&h.text, cit.chapter, cit.verse);
                                                        let v_res = librarian_core::QuoteGuard::verify_quote(cit.clone(), surrounding, &canonical_verse);
                                                        verified_count += 1;
                                                        if let Ok(v_json) = serde_json::to_string(&v_res) {
                                                            yield Ok(Event::default().event("verification").data(v_json));
                                                        }
                                                    }
                                                }
                                                if verified_count > 0 {
                                                    if let Some(ev) = make_office_event(
                                                        "verify",
                                                        worker.id,
                                                        Some("desk"),
                                                        &format!("QuoteGuard verified {} citation(s)", verified_count),
                                                        serde_json::json!({ "verified": verified_count }),
                                                    ) {
                                                        yield Ok(ev);
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if is_completed {
                                break;
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

        if let Some(ev) = make_office_event(
            "done",
            worker.id,
            Some("desk"),
            "Response delivered",
            serde_json::json!({}),
        ) {
            yield Ok(ev);
        }
        yield Ok(Event::default().event("done").data("[DONE]"));
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
