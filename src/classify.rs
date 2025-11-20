// Helper function to classify content
pub fn classify_content(content: &str, filename: &str) -> String {
    let lower_content = content.to_lowercase();
    let lower_filename = filename.to_lowercase();

    let classification = if lower_content.contains("chapter") || lower_filename.contains("chapter") {
        "Chapter".to_string()
    } else if lower_content.contains("scene") || lower_filename.contains("scene") {
        "Scene".to_string()
    } else if lower_content.contains("character") || lower_filename.contains("character") {
        "Codex/Character".to_string()
    } else if lower_content.contains("object") || lower_filename.contains("object") {
        "Codex/Objects".to_string()
    } else if lower_content.contains("location") || lower_filename.contains("location") {
        "Codex/Location".to_string()
    } else if lower_content.contains("time") || lower_filename.contains("time") {
        "Codex/Time".to_string()
    } else if lower_content.contains("story summary") || lower_filename.contains("summary") {
        "Codex/Story Summary".to_string()
    } else if lower_content.contains("outline") || lower_filename.contains("outline") {
        "Hierarchy".to_string()
    } else if lower_content.contains("plot") || lower_filename.contains("plot") {
        "Plot".to_string()
    } else if lower_content.contains("note") || lower_filename.contains("note") {
        "Notes".to_string()
    } else if lower_content.contains("research") || lower_filename.contains("research") {
        "Research".to_string()
    } else if lower_content.contains("analysis") || lower_filename.contains("analysis") {
        "Analysis".to_string()
    } else if lower_filename.ends_with(".jpg") || lower_filename.ends_with(".png") || lower_filename.ends_with(".gif") {
        "Images".to_string()
    } else {
        "Manuscript".to_string()
    };
    eprintln!("Classified {} (content length: {}) as: {}", filename, content.len(), classification);

    classification
}