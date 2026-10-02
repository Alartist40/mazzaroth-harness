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
    // 1. Check SQLite database documents
    match state.db.get_document(&doc_id) {
        Ok(Some(doc)) => return Ok(Json(doc)),
        Ok(None) => {}, // Fall through to dynamic scripture resolution
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // 2. Check dynamic scripture book resolution
    // Supported formats: "scripture:lang:version:Book" or "scripture-lang-version-Book"
    let parts: Vec<&str> = if doc_id.contains(':') {
        doc_id.split(':').collect()
    } else if doc_id.starts_with("scripture-") {
        doc_id.strip_prefix("scripture-").unwrap().split('-').collect()
    } else {
        Vec::new()
    };

    if parts.len() >= 4 && parts[0] == "scripture" {
        let lang = parts[1];
        let version = parts[2];
        let book = parts[3..].join(":").replace('_', " ");
        if let Ok(doc) = state.scripture.get_book_as_document(lang, version, &book) {
            return Ok(Json(doc));
        }
    } else if parts.len() == 3 && parts[0] == "scripture" {
        let lang = parts[1];
        let version = parts[2];
        if let Ok(meta) = state.scripture.get_metadata(lang, version) {
            if let Some(first_book) = meta.books.first() {
                if let Ok(doc) = state.scripture.get_book_as_document(lang, version, &first_book.name) {
                    return Ok(Json(doc));
                }
            }
        }
    } else if parts.len() >= 3 && parts[0] != "scripture" {
        let lang = parts[0];
        let version = parts[1];
        let book = parts[2..].join("-").replace('_', " ");
        if let Ok(doc) = state.scripture.get_book_as_document(lang, version, &book) {
            return Ok(Json(doc));
        }
    }

    Err(StatusCode::NOT_FOUND)
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

    // Integrate primary scripture catalog into /api/tree
    let scripture_langs = state.scripture.get_languages_detailed();
    for lang_info in &scripture_langs {
        let lang_code = &lang_info.code;
        if lang_code == "eng" || lang_code == "en" {
            for ver in &lang_info.versions {
                if let Ok(meta) = state.scripture.get_metadata(lang_code, ver) {
                    let books_as_chapters: Vec<KnowledgeTreeChapter> = meta
                        .books
                        .into_iter()
                        .enumerate()
                        .map(|(b_idx, b)| {
                            KnowledgeTreeChapter {
                                id: format!("book-{:02}", b_idx + 1),
                                title: b.name.clone(),
                                section_count: b.chapters,
                                sections: (1..=b.chapters)
                                    .map(|c| KnowledgeTreeSection {
                                        id: format!("ch-{:02}", c),
                                        title: format!("Chapter {}", c),
                                    })
                                    .collect(),
                            }
                        })
                        .collect();

                    let doc_tree = KnowledgeTreeDoc {
                        id: format!("scripture:{}:{}", lang_code, ver),
                        title: format!("Holy Bible ({}, {})", ver.to_uppercase(), lang_info.name),
                        category: "scripture".to_string(),
                        language: lang_code.clone(),
                        license: "public-domain".to_string(),
                        publisher: format!("Public Domain ({})", ver.to_uppercase()),
                        chapters: books_as_chapters,
                    };

                    tree_map
                        .entry("scripture".to_string())
                        .or_default()
                        .entry(lang_code.clone())
                        .or_default()
                        .push(doc_tree);
                }
            }
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

