//! US Army Field Manual to Librarian Box Content JSON converter
use serde_json::json;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run --bin ingest-fm <input_text_file> <output_json_file>");
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let text = fs::read_to_string(input_path)?;

    let doc = json!({
        "id": "fm-manual",
        "title": "US Army Field Manual",
        "category": "survival",
        "language": "en",
        "provenance": {
            "source": "US Government Printing Office (govinfo.gov)",
            "publisher": "Headquarters, Department of the Army",
            "license": "public-domain",
            "license_url": "https://www.usa.gov/government-works",
            "retrieved_date": "2026-09-28",
            "notes": "Official public domain United States Government publication"
        },
        "structure": [
            {
                "id": "ch-01",
                "title": "Chapter 1",
                "sections": [
                    {
                        "id": "ch-01-s-01",
                        "title": "General",
                        "text": text
                    }
                ]
            }
        ]
    });

    fs::write(output_path, serde_json::to_string_pretty(&doc)?)?;
    println!("Successfully exported FM document to {}", output_path);
    Ok(())
}
