use regex::Regex;
use std::collections::HashMap;
use std::fs;
use tracing::info;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let cards_dir = "/path/to/your/cards-database/data";

    info!("📊 Building name -> dexId map...");
    let name_to_dexid = build_name_to_dexid_map(cards_dir)?;

    info!("✅ Found {} Pokémon with dexId\n", name_to_dexid.len());

    info!("🔄 Updating files without dexId...");
    let updated = update_files_without_dexid(cards_dir, &name_to_dexid)?;

    info!("✨ Done! Updated {} files", updated);

    Ok(())
}

fn build_name_to_dexid_map(
    dir: &str,
) -> Result<HashMap<String, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ts") {
            let content = fs::read_to_string(path)?;
            if let Some(name_en) = extract_name_en(&content) {
                if let Some(dex_id) = extract_dex_id(&content) {
                    map.insert(name_en, dex_id);
                }
            }
        }
    }

    Ok(map)
}

fn update_files_without_dexid(
    dir: &str,
    name_to_dexid: &HashMap<String, Vec<i32>>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut updated_count = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.display().to_string().contains("Pokémon TCG Pocket") {
            //warn!("skip tcg pocket folder: {} ", path.display());
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("ts") {
            let content = fs::read_to_string(path)?;
            if extract_dex_id(&content).is_some() {
                continue;
            }
            if let Some(mut name_en) = extract_name_en(&content) {
                //Check if name contains mega word
                if name_en.contains("Mega") {
                    if let Some(second_word) = name_en.split_whitespace().nth(1) {
                        name_en = second_word.to_string();
                    }
                }
                //Check if it's an ex card
                if name_en.contains(" ex") {
                    if let Some(second_word) = name_en.split_whitespace().nth(0) {
                        name_en = second_word.to_string();
                    }
                }

                if let Some(dex_id) = name_to_dexid.get(&name_en) {
                    let updated_content = add_dex_id(&content, dex_id);
                    fs::write(path, updated_content)?;
                    info!("  ✓ Updated: {} (dexId: {:?})", path.display(), dex_id);
                    updated_count += 1;
                }
            }
        }
    }

    Ok(updated_count)
}

fn extract_name_en(content: &str) -> Option<String> {
    let re = Regex::new(r#"name:\s*\{[^}]*en:\s*"([^"]+)""#).unwrap();
    re.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

fn extract_dex_id(content: &str) -> Option<Vec<i32>> {
    let re = Regex::new(r"dexId:\s*\[([^\]]+)\]").unwrap();
    re.captures(content).and_then(|cap| {
        cap.get(1).map(|m| {
            m.as_str()
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect()
        })
    })
}

fn add_dex_id(content: &str, dex_id: &[i32]) -> String {
    // Find where to insert dexId (after "stage" or before "attacks")
    let dex_id_str = format!(
        "\tdexId: [{}],",
        dex_id
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Try to insert after "stage:"
    let stage_re = Regex::new(r#"(stage:\s*"[^"]+",)\n"#).unwrap();
    if let Some(_caps) = stage_re.captures(content) {
        return stage_re
            .replace(content, format!("$1\n{}\n", dex_id_str))
            .to_string();
    }

    // Otherwise insert before "attacks:"
    let attacks_re = Regex::new(r"(\tattacks:)").unwrap();
    if attacks_re.is_match(content) {
        return attacks_re
            .replace(content, format!("{}\n$1", dex_id_str))
            .to_string();
    }

    content.to_string()
}
