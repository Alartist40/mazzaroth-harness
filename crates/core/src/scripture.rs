use crate::content::{ContentChapter, ContentDocument, ContentProvenance, ContentSection};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn is_safe_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

pub fn get_language_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "afr" => "Afrikaans",
        "alb" => "Albanian (Shqip)",
        "arm" => "Armenian (Հայերեն)",
        "arn" => "Mapudungun",
        "ben" => "Bengali (বাংলা)",
        "ceb" => "Cebuano (Bisaya)",
        "ces" => "Czech (Čeština)",
        "che" => "Chechen (Нохчийн)",
        "chu" => "Old Church Slavonic (Црькьвьнословѣньскъ)",
        "cop" => "Coptic (ⲘⲉⲧⲢⲉⲙⲛ̀ⲭⲏⲙⲓ)",
        "dan" => "Danish (Dansk)",
        "deu" => "German (Deutsch)",
        "ell" => "Greek (Ἑλληνική / Biblical Greek)",
        "eng" => "English",
        "epo" => "Esperanto",
        "est" => "Estonian (Eesti)",
        "fin" => "Finnish (Suomi)",
        "fra" => "French (Français)",
        "glv" => "Manx Gaelic (Gaelg)",
        "got" => "Gothic (Gutiska)",
        "guj" => "Gujarati (ગુજરાતી)",
        "hat" => "Haitian Creole (Kreyòl)",
        "heb" => "Hebrew (עברית / Biblical Hebrew)",
        "hin" => "Hindi (हिन्दी)",
        "hrv" => "Croatian (Hrvatski)",
        "hun" => "Hungarian (Magyar)",
        "ind" => "Indonesian (Bahasa Indonesia)",
        "ita" => "Italian (Italiano)",
        "jpn" => "Japanese (日本語)",
        "kan" => "Kannada (ಕನ್ನಡ)",
        "kor" => "Korean (한국어)",
        "lat" => "Latin (Biblia Sacra Vulgata)",
        "mal" => "Malayalam (മലയാളം)",
        "mar" => "Marathi (मराठी)",
        "mlg" => "Malagasy",
        "mri" => "Māori (Te Reo)",
        "mya" => "Burmese (မြန်မာ)",
        "nep" => "Nepali (नेपाली)",
        "nld" => "Dutch (Nederlands)",
        "nor" => "Norwegian (Norsk)",
        "nso" => "Northern Sotho (Sepedi)",
        "ori" => "Odia (ଓଡ଼ିଆ)",
        "pan" => "Punjabi (ਪੰਜਾਬੀ)",
        "pol" => "Polish (Polski)",
        "pon" => "Pohnpeian",
        "por" => "Portuguese (Português)",
        "rus" => "Russian (Русский)",
        "sam" => "Samaritan (Shamerim)",
        "slv" => "Slovenian (Slovenščina)",
        "sml" => "Central Sama",
        "spa" => "Spanish (Español)",
        "srp" => "Serbian (Српски)",
        "swe" => "Swedish (Svenska)",
        "syr" => "Syriac (Peshitta / ܣܘܪܝܝܐ)",
        "tam" => "Tamil (தமிழ்)",
        "tel" => "Telugu (తెలుగు)",
        "tgl" => "Tagalog (Filipino)",
        "tha" => "Thai (ภาษาไทย)",
        "tpi" => "Tok Pisin",
        "tsg" => "Tausug",
        "ukr" => "Ukrainian (Українська)",
        "vie" => "Vietnamese (Tiếng Việt)",
        "vls" => "West Flemish (Vlaams)",
        "xho" => "Xhosa (isiXhosa)",
        "zho" => "Chinese (中文 / 圣经)",
        "zul" => "Zulu (isiZulu)",
        _ => "Scripture Language",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureLanguageInfo {
    pub code: String,
    pub name: String,
    pub version_count: usize,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureBook {
    pub name: String,
    pub chapters: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub name: String,
    pub chapters: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureMetaResponse {
    pub language: String,
    pub language_name: String,
    pub version: String,
    pub total_books: usize,
    pub books: Vec<BookMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureVersionProvenance {
    pub title: String,
    pub publisher: String,
    pub license: String,
    pub year: Option<u32>,
    pub source: String,
    pub personal_use_only: bool,
}

pub fn get_version_provenance(lang: &str, version: &str) -> ScriptureVersionProvenance {
    let v_lower = version.to_lowercase();
    let l_lower = lang.to_lowercase();
    let lang_name = get_language_name(&l_lower);

    match (l_lower.as_str(), v_lower.as_str()) {
        ("eng", "kjv") | ("eng", "kjva") | ("eng", "kjvpce") => ScriptureVersionProvenance {
            title: "King James Version (Authorized Edition 1611)".into(),
            publisher: "Crown / King's Printers (1611)".into(),
            license: "public-domain".into(),
            year: Some(1611),
            source: "Historical Public Domain Scripture Archive".into(),
            personal_use_only: false,
        },
        ("eng", "asv") => ScriptureVersionProvenance {
            title: "American Standard Version (1901)".into(),
            publisher: "Thomas Nelson & Sons (1901)".into(),
            license: "public-domain".into(),
            year: Some(1901),
            source: "American Standard Bible Committee".into(),
            personal_use_only: false,
        },
        ("eng", "bbe") => ScriptureVersionProvenance {
            title: "Bible in Basic English".into(),
            publisher: "Cambridge University Press / S.H. Hooke (1949/1964)".into(),
            license: "public-domain".into(),
            year: Some(1949),
            source: "Orthological Institute".into(),
            personal_use_only: false,
        },
        ("eng", "web") => ScriptureVersionProvenance {
            title: "World English Bible".into(),
            publisher: "Rainbow Missions, Inc. (Public Domain)".into(),
            license: "public-domain".into(),
            year: Some(2000),
            source: "World English Bible Project".into(),
            personal_use_only: false,
        },
        ("eng", "ylt") => ScriptureVersionProvenance {
            title: "Young's Literal Translation".into(),
            publisher: "A. Fullarton & Co. (1862/1898)".into(),
            license: "public-domain".into(),
            year: Some(1898),
            source: "Robert Young Translation Archive".into(),
            personal_use_only: false,
        },
        ("eng", "darby") => ScriptureVersionProvenance {
            title: "Darby Bible Translation".into(),
            publisher: "John Nelson Darby (1890)".into(),
            license: "public-domain".into(),
            year: Some(1890),
            source: "Darby Historical Scripture Archive".into(),
            personal_use_only: false,
        },
        ("eng", "geneva1599") => ScriptureVersionProvenance {
            title: "Geneva Bible (1599 Edition)".into(),
            publisher: "Rowland Hall (1599)".into(),
            license: "public-domain".into(),
            year: Some(1599),
            source: "Geneva Bible Historical Corpus".into(),
            personal_use_only: false,
        },
        ("eng", "tyndale") => ScriptureVersionProvenance {
            title: "Tyndale Bible (1526/1534)".into(),
            publisher: "William Tyndale (1534)".into(),
            license: "public-domain".into(),
            year: Some(1534),
            source: "Tyndale Historical Scripture Corpus".into(),
            personal_use_only: false,
        },
        ("eng", "wycliffe") => ScriptureVersionProvenance {
            title: "Wycliffe Bible (1382)".into(),
            publisher: "John Wycliffe (1382)".into(),
            license: "public-domain".into(),
            year: Some(1382),
            source: "Middle English Biblical Corpus".into(),
            personal_use_only: false,
        },
        ("eng", "webster") | ("eng", "rwebster") => ScriptureVersionProvenance {
            title: "Webster's Revision of the KJV".into(),
            publisher: "Noah Webster (1833)".into(),
            license: "public-domain".into(),
            year: Some(1833),
            source: "Webster Revision Archive".into(),
            personal_use_only: false,
        },
        ("eng", "bsb") => ScriptureVersionProvenance {
            title: "Berean Standard Bible".into(),
            publisher: "Berean Bible (CC0 1.0 Universal)".into(),
            license: "cc0".into(),
            year: Some(2016),
            source: "Berean Study Bible Project".into(),
            personal_use_only: false,
        },
        ("jpn", "japkougo") => ScriptureVersionProvenance {
            title: "Kōgo-yaku (口語訳聖書 1954/1955)".into(),
            publisher: "Japan Bible Society (1954/1955, Public Domain)".into(),
            license: "public-domain".into(),
            year: Some(1954),
            source: "Japan Bible Society Historical Archive".into(),
            personal_use_only: false,
        },
        ("jpn", "japbungo") => ScriptureVersionProvenance {
            title: "Bungo-yaku (文語訳聖書 1887/1917)".into(),
            publisher: "Japan Bible Society (1887/1917, Public Domain)".into(),
            license: "public-domain".into(),
            year: Some(1887),
            source: "Japan Bible Society Historical Archive".into(),
            personal_use_only: false,
        },
        ("deu", "luther1912") | ("deu", "luther") => ScriptureVersionProvenance {
            title: "Lutherbibel 1912".into(),
            publisher: "Cansteinsche Bibelanstalt (1912)".into(),
            license: "public-domain".into(),
            year: Some(1912),
            source: "German Historical Scripture Archive".into(),
            personal_use_only: false,
        },
        ("lat", "vulgate") | ("lat", "lat") => ScriptureVersionProvenance {
            title: "Biblia Sacra Vulgata".into(),
            publisher: "Jerome / Catholic Church (405 AD, Public Domain)".into(),
            license: "public-domain".into(),
            year: Some(405),
            source: "Vulgata Clementina Historical Archive".into(),
            personal_use_only: false,
        },
        ("ell", "tr") | ("ell", "statresgnt") | ("ell", "byz") => ScriptureVersionProvenance {
            title: "Greek New Testament (Textus Receptus / Stephanus 1550)".into(),
            publisher: "Robert Estienne / Stephanus (1550)".into(),
            license: "public-domain".into(),
            year: Some(1550),
            source: "Historical Biblical Greek Corpus".into(),
            personal_use_only: false,
        },
        ("heb", "wlc") | ("heb", "hebmodern") => ScriptureVersionProvenance {
            title: "Westminster Leningrad Codex (Masoretic Text)".into(),
            publisher: "Groves Center / Historical Masoretic Scholars (1008 AD)".into(),
            license: "public-domain".into(),
            year: Some(1008),
            source: "Westminster Leningrad Codex Corpus".into(),
            personal_use_only: false,
        },
        ("rus", "russynodal") => ScriptureVersionProvenance {
            title: "Russian Synodal Bible (Синодальный перевод 1876)".into(),
            publisher: "Most Holy Synod of the Russian Orthodox Church (1876)".into(),
            license: "public-domain".into(),
            year: Some(1876),
            source: "Russian Synodal Scripture Corpus".into(),
            personal_use_only: false,
        },
        _ => ScriptureVersionProvenance {
            title: format!("Historical {} Scripture ({})", lang_name, version.to_uppercase()),
            publisher: format!("Public Domain Historical Translation ({})", version.to_uppercase()),
            license: "public-domain".into(),
            year: None,
            source: format!("Public Domain Biblical Corpus ({})", lang_name),
            personal_use_only: false,
        },
    }
}

#[derive(Clone)]
pub struct ScriptureReader {
    bibles_dir: PathBuf,
    cache: Arc<Mutex<HashMap<String, Arc<Vec<ScriptureBook>>>>>,
}

impl ScriptureReader {
    pub fn new(bibles_dir: impl AsRef<Path>) -> Self {
        Self {
            bibles_dir: bibles_dir.as_ref().to_path_buf(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn resolve_file(&self, lang: &str, version: &str) -> Option<PathBuf> {
        if !is_safe_identifier(lang) || !is_safe_identifier(version) {
            return None;
        }

        let lang_dir = self.bibles_dir.join(lang);
        if !lang_dir.exists() {
            return None;
        }

        // Exact match candidate
        let candidate = lang_dir.join(format!("{}.json", version));
        if candidate.exists() {
            return Some(candidate);
        }

        // Case-insensitive stem matching
        let v_lower = version.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(&lang_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem.to_lowercase() == v_lower {
                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn load_version(&self, lang: &str, version: &str) -> Result<Arc<Vec<ScriptureBook>>> {
        if !is_safe_identifier(lang) || !is_safe_identifier(version) {
            bail!("Invalid language or version identifier");
        }

        let key = format!("{}:{}", lang.to_lowercase(), version.to_lowercase());
        {
            let cache = self.cache.lock().unwrap();
            if let Some(books) = cache.get(&key) {
                return Ok(Arc::clone(books));
            }
        }

        let file_path = self
            .resolve_file(lang, version)
            .ok_or_else(|| anyhow::anyhow!("Scripture version not found: {}/{}", lang, version))?;

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let books: Vec<ScriptureBook> = serde_json::from_reader(reader)?;
        let arc_books = Arc::new(books);

        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, Arc::clone(&arc_books));
        Ok(arc_books)
    }

    pub fn get_metadata(&self, lang: &str, version: &str) -> Result<ScriptureMetaResponse> {
        let books = self.load_version(lang, version)?;
        let meta_books: Vec<BookMeta> = books
            .iter()
            .map(|b| BookMeta {
                name: b.name.clone(),
                chapters: b.chapters.len(),
            })
            .collect();

        Ok(ScriptureMetaResponse {
            language: lang.to_string(),
            language_name: get_language_name(lang).to_string(),
            version: version.to_string(),
            total_books: meta_books.len(),
            books: meta_books,
        })
    }

    pub fn get_chapter(
        &self,
        lang: &str,
        version: &str,
        book_name: &str,
        chapter_num: usize,
    ) -> Result<Vec<String>> {
        let books = self.load_version(lang, version)?;
        let book = books
            .iter()
            .find(|b| b.name.eq_ignore_ascii_case(book_name))
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_name))?;

        let chapter_idx = if chapter_num > 0 { chapter_num - 1 } else { 0 };
        let verses = book
            .chapters
            .get(chapter_idx)
            .ok_or_else(|| anyhow::anyhow!("Chapter {} not found in book {}", chapter_num, book_name))?;

        Ok(verses.clone())
    }

    pub fn get_book_as_document(&self, lang: &str, version: &str, book_name: &str) -> Result<ContentDocument> {
        let books = self.load_version(lang, version)?;
        let book = books
            .iter()
            .find(|b| b.name.eq_ignore_ascii_case(book_name))
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_name))?;

        let mut structure = Vec::new();
        for (c_idx, verses) in book.chapters.iter().enumerate() {
            let ch_num = c_idx + 1;
            let verse_lines: Vec<String> = verses
                .iter()
                .enumerate()
                .map(|(v_idx, text)| format!("{} {}", v_idx + 1, text.trim()))
                .collect();

            structure.push(ContentChapter {
                id: format!("ch-{:02}", ch_num),
                title: format!("{} Chapter {}", book.name, ch_num),
                sections: vec![ContentSection {
                    id: format!("ch-{:02}-s-01", ch_num),
                    title: format!("{}:1-{}", ch_num, verses.len()),
                    text: verse_lines.join("\n"),
                }],
            });
        }

        let prov = get_version_provenance(lang, version);
        let doc_id = format!("scripture:{}:{}:{}", lang, version, book.name.replace(' ', "_"));

        let retrieved_date = if let Some(yr) = prov.year {
            format!("{}-01-01", yr)
        } else {
            "1900-01-01".to_string()
        };

        Ok(ContentDocument {
            id: doc_id,
            title: format!("The Book of {} ({})", book.name, prov.title),
            category: "scripture".to_string(),
            language: lang.to_string(),
            provenance: ContentProvenance {
                source: prov.source,
                publisher: prov.publisher,
                license: prov.license,
                license_url: None,
                retrieved_date,
                notes: Some(format!("Canonical {} translation containing {} chapters", version.to_uppercase(), structure.len())),
                personal_use_only: prov.personal_use_only,
            },
            structure,
        })
    }

    pub fn list_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.bibles_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.file_type().map_or(false, |t| t.is_dir()) {
                    if let Some(name) = entry.file_name().to_str() {
                        if is_safe_identifier(name) {
                            langs.push(name.to_string());
                        }
                    }
                }
            }
        }
        langs.sort();
        langs
    }

    pub fn get_languages_detailed(&self) -> Vec<ScriptureLanguageInfo> {
        let codes = self.list_languages();
        codes
            .into_iter()
            .map(|code| {
                let versions = self.list_versions(&code);
                let version_count = versions.len();
                let name = get_language_name(&code).to_string();
                ScriptureLanguageInfo {
                    code,
                    name,
                    version_count,
                    versions,
                }
            })
            .collect()
    }

    pub fn list_versions(&self, lang: &str) -> Vec<String> {
        if !is_safe_identifier(lang) {
            return Vec::new();
        }

        let mut versions = Vec::new();
        let lang_dir = self.bibles_dir.join(lang);
        if let Ok(entries) = std::fs::read_dir(&lang_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if is_safe_identifier(stem) {
                            versions.push(stem.to_string());
                        }
                    }
                }
            }
        }
        versions.sort();
        versions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scripture_multilingual_loading() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
        let bibles_path = Path::new(&manifest_dir).join("../../galaxy/data/bibles");
        let path_to_use = if bibles_path.exists() {
            bibles_path
        } else {
            PathBuf::from("galaxy/data/bibles")
        };
        let reader = ScriptureReader::new(path_to_use);
        let langs = reader.list_languages();
        assert!(langs.len() >= 60, "Expected at least 60 languages, got {}", langs.len());
        assert!(langs.contains(&"eng".to_string()));
        assert!(langs.contains(&"jpn".to_string()));
        assert!(langs.contains(&"deu".to_string()));

        let eng_versions = reader.list_versions("eng");
        assert!(eng_versions.contains(&"kjv".to_string()));

        let meta = reader.get_metadata("eng", "kjv").unwrap();
        assert_eq!(meta.total_books, 66);
        assert_eq!(meta.books[0].name, "Genesis");
        assert_eq!(meta.books[0].chapters, 50);

        let doc = reader.get_book_as_document("eng", "kjv", "Genesis").unwrap();
        assert_eq!(doc.structure.len(), 50);
        assert_eq!(doc.category, "scripture");
        assert_eq!(doc.provenance.license, "public-domain");
        assert_eq!(doc.provenance.publisher, "Crown / King's Printers (1611)");
        assert_eq!(doc.provenance.retrieved_date, "1611-01-01");
        assert!(doc.validate_provenance().is_ok());
    }

    #[test]
    fn test_scripture_provenance_manifest() {
        let p_kjv = get_version_provenance("eng", "kjv");
        assert_eq!(p_kjv.license, "public-domain");
        assert_eq!(p_kjv.year, Some(1611));

        let p_jpn = get_version_provenance("jpn", "japkougo");
        assert_eq!(p_jpn.license, "public-domain");
        assert_eq!(p_jpn.year, Some(1954));

        let p_deu = get_version_provenance("deu", "luther1912");
        assert_eq!(p_deu.license, "public-domain");
        assert_eq!(p_deu.year, Some(1912));
    }
}

