use std::path::Path;

// Helper function to convert file to Markdown based on extension
pub fn convert_file_to_markdown(file_path: &Path) -> Result<String, String> {
    use std::fs::read_to_string;
    
    let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    match extension.to_lowercase().as_str() {
        "md" => {
            let content = read_to_string(file_path).map_err(|e| format!("Failed to read MD file: {}", e))?;
            Ok(content)
        }
        _ => {
            Err(format!("Unsupported file extension: {}", extension))
        },
    }
}