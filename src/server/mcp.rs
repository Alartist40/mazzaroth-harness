use crate::cognitive::MemoryTier;
use crate::server::http::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct McpRpcRequest {
    pub jsonrpc: Option<String>,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

pub async fn handle_mcp_post(
    State(state): State<ServerState>,
    Json(req): Json<McpRpcRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let req_id = req.id.clone().unwrap_or(json!(1));

    match req.method.as_str() {
        "tools/list" => {
            let tools = json!({
                "tools": [
                    {
                        "name": "mazzaroth_remember",
                        "description": "Ingests and anchors a thought, perception, or factual knowledge into Mazzaroth celestial memory.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "label": { "type": "string", "description": "Short concept label or entity title" },
                                "content": { "type": "string", "description": "Full contextual text or episode description" },
                                "tier": { "type": "string", "enum": ["working", "episodic", "semantic", "celestial"] },
                                "tags": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["label", "content"]
                        }
                    },
                    {
                        "name": "mazzaroth_recall",
                        "description": "Associatively searches and recalls contextual memories from Mazzaroth 4-tier knowledge graph using FTS5 and Hebbian links.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "query": { "type": "string", "description": "Keyword or semantic search query" },
                                "limit": { "type": "number", "description": "Maximum number of memory stars to return" }
                            },
                            "required": ["query"]
                        }
                    },
                    {
                        "name": "mazzaroth_get_galaxy",
                        "description": "Returns current 3D celestial coordinates, orbits, and constellation links for visual representation.",
                        "inputSchema": { "type": "object", "properties": {} }
                    }
                ]
            });

            Ok(Json(json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "result": tools
            })))
        }
        "tools/call" => {
            let params = req.params.unwrap_or(json!({}));
            let tool_name = params["name"].as_str().unwrap_or_default();
            let args = &params["arguments"];

            match tool_name {
                "mazzaroth_remember" => {
                    let label = args["label"].as_str().unwrap_or("Untitled");
                    let content = args["content"].as_str().unwrap_or("");
                    let tier_str = args["tier"].as_str().unwrap_or("episodic");
                    let tier = tier_str.parse::<MemoryTier>().ok().unwrap_or(MemoryTier::Episodic);
                    let tags: Vec<String> = args["tags"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default();

                    match state.engine.ingest(label, content, tier, tags) {
                        Ok(node) => Ok(Json(json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": {
                                "content": [{
                                    "type": "text",
                                    "text": format!("Memory formed: [{}] (id: {})", node.label, node.id)
                                }],
                                "node": node
                            }
                        }))),
                        Err(e) => Ok(Json(json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": { "code": -32000, "message": format!("Ingest error: {}", e) }
                        }))),
                    }
                }
                "mazzaroth_recall" => {
                    let query = args["query"].as_str().unwrap_or("");
                    let limit = args["limit"].as_u64().unwrap_or(5) as usize;

                    match state.engine.recall(query, limit) {
                        Ok(nodes) => {
                            let text_summary = nodes
                                .iter()
                                .map(|n| format!("✦ [{}] (Tier: {:?}): {}", n.label, n.tier, n.content))
                                .collect::<Vec<_>>()
                                .join("\n\n");

                            Ok(Json(json!({
                                "jsonrpc": "2.0",
                                "id": req_id,
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": if text_summary.is_empty() { "No related memories found.".to_string() } else { text_summary }
                                    }],
                                    "nodes": nodes
                                }
                            })))
                        }
                        Err(e) => Ok(Json(json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": { "code": -32000, "message": format!("Recall error: {}", e) }
                        }))),
                    }
                }
                "mazzaroth_get_galaxy" => match state.engine.get_galaxy_state() {
                    Ok(galaxy) => Ok(Json(json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": galaxy
                    }))),
                    Err(e) => Ok(Json(json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "error": { "code": -32000, "message": format!("Galaxy error: {}", e) }
                    }))),
                },
                _ => Ok(Json(json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": { "code": -32601, "message": format!("Unknown tool: {}", tool_name) }
                }))),
            }
        }
        _ => Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "error": { "code": -32601, "message": format!("Method not found: {}", req.method) }
        }))),
    }
}
