use crate::state::ServerState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use librarian_core::{ContentDocument, DocumentSummary};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTreeNode {
    pub category: String,
    pub languages: Vec<KnowledgeTreeLanguage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTreeLanguage {
    pub language: String,
    pub documents: Vec<KnowledgeTreeDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTreeDoc {
    pub id: String,
    pub title: String,
    pub category: String,
    pub language: String,
    pub license: String,
    pub publisher: String,
    pub chapters: Vec<KnowledgeTreeChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTreeChapter {
    pub id: String,
    pub title: String,
    pub section_count: usize,
    pub sections: Vec<KnowledgeTreeSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTreeSection {
    pub id: String,
    pub title: String,
}

pub async fn handle_list_documents(
    State(state): State<ServerState>,
) -> Result<Json<Vec<DocumentSummary>>, StatusCode> {
    state
        .db
        .list_documents()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_list_categories(
    State(state): State<ServerState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    state
        .db
        .list_categories()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_get_document(
    State(state): State<ServerState>,
    Path(doc_id): Path<String>,
) -> Result<Json<ContentDocument>, StatusCode> {
    match state.db.get_document(&doc_id) {
        Ok(Some(doc)) => Ok(Json(doc)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_get_knowledge_tree(
    State(state): State<ServerState>,
) -> Result<Json<Vec<KnowledgeTreeNode>>, StatusCode> {
    let summaries = state
        .db
        .list_documents()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Grouping: Category -> Language -> Document
    let mut tree_map: BTreeMap<String, BTreeMap<String, Vec<KnowledgeTreeDoc>>> = BTreeMap::new();

    for summary in summaries {
        if let Ok(Some(full_doc)) = state.db.get_document(&summary.id) {
            let chapters = full_doc
                .structure
                .into_iter()
                .map(|ch| {
                    let section_count = ch.sections.len();
                    let sections = ch
                        .sections
                        .into_iter()
                        .map(|s| KnowledgeTreeSection {
                            id: s.id,
                            title: s.title,
                        })
                        .collect();
                    KnowledgeTreeChapter {
                        id: ch.id,
                        title: ch.title,
                        section_count,
                        sections,
                    }
                })
                .collect();

            let doc_tree = KnowledgeTreeDoc {
                id: summary.id,
                title: summary.title,
                category: summary.category.clone(),
                language: summary.language.clone(),
                license: summary.license,
                publisher: summary.publisher,
                chapters,
            };

            tree_map
                .entry(summary.category)
                .or_default()
                .entry(summary.language)
                .or_default()
                .push(doc_tree);
        }
    }

    let mut result = Vec::new();
    for (cat_name, lang_map) in tree_map {
        let mut languages = Vec::new();
        for (lang_name, docs) in lang_map {
            languages.push(KnowledgeTreeLanguage {
                language: lang_name,
                documents: docs,
            });
        }
        result.push(KnowledgeTreeNode {
            category: cat_name,
            languages,
        });
    }

    Ok(Json(result))
}

