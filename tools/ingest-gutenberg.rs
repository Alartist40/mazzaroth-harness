//! Project Gutenberg Plain Text to Librarian Box Content JSON converter
use serde_json::json;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run --bin ingest-gutenberg <input_text_file> <title> <output_json_file>");
        return Ok(());
    }

    let input_path = &args[1];
    let title = &args[2];
    let output_path = &args[3];
    let text = fs::read_to_string(input_path)?;

    let doc = json!({
        "id": format!("gutenberg-{}", title.to_lowercase().replace(' ', "-")),
        "title": title,
        "category": "literature",
        "language": "en",
        "provenance": {
            "source": "Project Gutenberg (gutenberg.org)",
            "publisher": "Project Gutenberg Literary Archive Foundation",
            "license": "gutenberg",
            "license_url": "https://www.gutenberg.org/license",
            "retrieved_date": "2026-09-28",
            "notes": "Public domain literary work digitized by Project Gutenberg"
        },
        "structure": [
            {
                "id": "ch-01",
                "title": "Complete Work",
                "sections": [
                    {
                        "id": "sec-01",
                        "title": title,
                        "text": text
                    }
                ]
            }
        ]
    });

    fs::write(output_path, serde_json::to_string_pretty(&doc)?)?;
    println!("Successfully exported Gutenberg document to {}", output_path);
    Ok(())
}
