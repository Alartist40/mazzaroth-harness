use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

fn floor_char_boundary(s: &str, mut index: usize) -> usize {
    if index >= s.len() {
        return s.len();
    }
    while !s.is_char_boundary(index) {
        if index == 0 {
            break;
        }
        index -= 1;
    }
    index
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProvenance {
    pub source: String,
    pub publisher: String,
    pub license: String,
    #[serde(default)]
    pub license_url: Option<String>,
    pub retrieved_date: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub personal_use_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSection {
    pub id: String,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChapter {
    pub id: String,
    pub title: String,
    pub sections: Vec<ContentSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDocument {
    pub id: String,
    pub title: String,
    pub category: String,
    pub language: String,
    pub provenance: ContentProvenance,
    pub structure: Vec<ContentChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub doc_id: String,
    pub section_id: String,
    pub title_path: String,
    pub text: String,
}

impl ContentDocument {
    /// Strict provenance validator: rejects missing or legally ambiguous fields
    pub fn validate_provenance(&self) -> Result<()> {
        if self.id.trim().is_empty() {
            bail!("Document id cannot be empty");
        }
        if self.title.trim().is_empty() {
            bail!("Document title cannot be empty");
        }
        if self.category.trim().is_empty() {
            bail!("Document category cannot be empty");
        }
        if self.language.trim().is_empty() {
            bail!("Document language cannot be empty");
        }

        let p = &self.provenance;
        if p.source.trim().is_empty() {
            bail!("Provenance source is required");
        }
        if p.publisher.trim().is_empty() {
            bail!("Provenance publisher is required");
        }
        if p.license.trim().is_empty() {
            bail!("Provenance license is required");
        }
        if p.retrieved_date.trim().is_empty() {
            bail!("Provenance retrieved_date is required");
        }

        // Validate license against known permissive/public-domain standards
        let l = p.license.to_lowercase();
        let valid_license = l.contains("public domain")
            || l.contains("public-domain")
            || l.contains("cc0")
            || l.contains("cc-by")
            || l.contains("mit")
            || l.contains("apache")
            || l.contains("us government work")
            || l.contains("open source")
            || p.personal_use_only;

        if !valid_license {
            bail!(
                "License '{}' is not recognized as public domain or permissive. If personal, set personal_use_only: true",
                p.license
            );
        }

        if self.structure.is_empty() {
            bail!("Document must contain at least one chapter");
        }

        Ok(())
    }

    pub fn to_chunks(&self) -> Vec<ContentChunk> {
        let mut chunks = Vec::new();

        for chapter in &self.structure {
            for section in &section_list(&chapter.sections) {
                let title_path = format!("{} > {} > {}", self.title, chapter.title, section.title);
                
                let text = section.text.trim();
                if text.len() <= 3200 {
                    chunks.push(ContentChunk {
                        doc_id: self.id.clone(),
                        section_id: section.id.clone(),
                        title_path,
                        text: text.to_string(),
                    });
                } else {
                    let mut start = 0;
                    let mut chunk_idx = 1;
                    while start < text.len() {
                        let target_end = floor_char_boundary(text, (start + 3000).min(text.len()));
                        let mut end = target_end;
                        
                        // Try to split on sentence or newline boundary in the latter half of the chunk
                        if end < text.len() {
                            let slice_search = &text[start..end];
                            if let Some(pos) = slice_search.rfind("\n\n") {
                                if pos >= 1000 {
                                    end = floor_char_boundary(text, start + pos + 2);
                                }
                            } else if let Some(pos) = slice_search.rfind('\n') {
                                if pos >= 1000 {
                                    end = floor_char_boundary(text, start + pos + 1);
                                }
                            } else if let Some(pos) = slice_search.rfind(". ") {
                                if pos >= 1000 {
                                    end = floor_char_boundary(text, start + pos + 2);
                                }
                            }
                        }

                        let slice = text[start..end].trim();
                        if !slice.is_empty() {
                            chunks.push(ContentChunk {
                                doc_id: self.id.clone(),
                                section_id: format!("{}-part{}", section.id, chunk_idx),
                                title_path: format!("{} (Part {})", title_path, chunk_idx),
                                text: slice.to_string(),
                            });
                        }
                        chunk_idx += 1;
                        if end >= text.len() {
                            break;
                        }
                        // Advance to end of current chunk
                        start = end;
                    }
                }
            }
        }

        chunks
    }
}

fn section_list(sections: &[ContentSection]) -> Vec<&ContentSection> {
    sections.iter().collect()
}
