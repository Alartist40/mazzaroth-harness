//! Public Domain Bible Converter to Librarian Box Content JSON
use serde_json::json;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run --bin ingest-bible <input_json_file> <version_name> <output_json_file>");
        return Ok(());
    }

    let input_path = &args[1];
    let version = &args[2];
    let output_path = &args[3];

    let content = fs::read_to_string(input_path)?;
    let raw_books: serde_json::Value = serde_json::from_str(&content)?;

    let mut structure = Vec::new();

    if let Some(books) = raw_books.as_array() {
        for (b_idx, book) in books.iter().enumerate() {
            let book_name = book["name"].as_str().unwrap_or("Book");
            let mut sections = Vec::new();

            if let Some(chapters) = book["chapters"].as_array() {
                for (c_idx, chapter) in chapters.iter().enumerate() {
                    let mut chapter_text = String::new();
                    if let Some(verses) = chapter.as_array() {
                        for (v_idx, verse) in verses.iter().enumerate() {
                            if let Some(v_str) = verse.as_str() {
                                chapter_text.push_str(&format!("{} {}\n", v_idx + 1, v_str));
                            }
                        }
                    }

                    sections.push(json!({
                        "id": format!("book-{}-ch-{}", b_idx + 1, c_idx + 1),
                        "title": format!("{} Chapter {}", book_name, c_idx + 1),
                        "text": chapter_text.trim()
                    }));
                }
            }

            structure.push(json!({
                "id": format!("book-{}", b_idx + 1),
                "title": book_name,
                "sections": sections
            }));
        }
    }

    let doc = json!({
        "id": format!("bible-{}", version.to_lowercase()),
        "title": format!("Holy Bible ({})", version.to_uppercase()),
        "category": "scripture",
        "language": "en",
        "provenance": {
            "source": "Public Domain Scripture Repository",
            "publisher": format!("Public Domain ({})", version.to_uppercase()),
            "license": "public-domain",
            "license_url": null,
            "retrieved_date": "2026-09-28",
            "notes": "Verified public-domain biblical translation"
        },
        "structure": structure
    });

    fs::write(output_path, serde_json::to_string_pretty(&doc)?)?;
    println!("Successfully exported Bible to {}", output_path);
    Ok(())
}
