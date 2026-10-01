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
        let l = p.license.trim().to_lowercase().replace('_', "-");
        
        // Explicitly reject non-commercial, no-derivatives, and proprietary restrictions
        if l.contains("-nc")
            || l.contains("non-commercial")
            || l.contains("noncommercial")
            || l.contains("-nd")
            || l.contains("no-derivatives")
            || l.contains("noderivatives")
            || l.contains("all rights reserved")
            || l.contains("proprietary")
            || l.contains("unlicensed")
            || l.contains("unknown")
        {
            bail!(
                "License '{}' contains restrictive terms (-nc, -nd, proprietary) and cannot be ingested as open sovereign corpus. For personal use, set personal_use_only: true and license: 'personal-use-only'",
                p.license
            );
        }

        let is_permissive_or_pd = matches!(
            l.as_str(),
            "public-domain"
                | "public domain"
                | "cc0"
                | "cc0-1.0"
                | "cc-by"
                | "cc-by-1.0"
                | "cc-by-2.0"
                | "cc-by-2.5"
                | "cc-by-3.0"
                | "cc-by-4.0"
                | "cc-by-sa"
                | "cc-by-sa-1.0"
                | "cc-by-sa-2.0"
                | "cc-by-sa-2.5"
                | "cc-by-sa-3.0"
                | "cc-by-sa-4.0"
                | "mit"
                | "apache-2.0"
                | "apache 2.0"
                | "bsd-2-clause"
                | "bsd-3-clause"
                | "us-government-work"
                | "us government work"
                | "gutenberg"
        );

        let is_valid_personal = p.personal_use_only && (l == "personal-use-only" || l == "personal");

        if !is_permissive_or_pd && !is_valid_personal {
            bail!(
                "License '{}' is not recognized as verified public domain or permissive open source. If personal, set personal_use_only: true and license: 'personal-use-only'",
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
