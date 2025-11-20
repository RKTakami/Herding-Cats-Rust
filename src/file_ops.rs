use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const MAX_CONTENT_SIZE: usize = 10 * 1024 * 1024; // 10MB limit

// use crate::file_access::log_file_operation; // TODO: Implement file_access module

/// Save document implementation with actual file operations
pub async fn save_document_impl(content: String, path: String) -> Result<(), String> {
    // Validate the path first for security
    validate_path(path.clone()).await?;

    // Validate content size
    if content.len() > MAX_CONTENT_SIZE {
        return Err("Document content exceeds maximum size limit (10MB)".to_string());
    }

    // Ensure the directory exists
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Write the file
    let mut file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write file: {}", e))?;

    // Log the operation
    // log_file_operation("write", "user", &file_path, true, None); // TODO: Implement

    println!("Document saved successfully to: {}", path);
    Ok(())
}

/// Load document implementation with actual file operations
pub async fn load_document_impl(path: String) -> Result<String, String> {
    // Validate the path first for security
    validate_path(path.clone()).await?;

    let file_path = Path::new(&path);

    if !file_path.exists() {
        return Err(format!("File not found: {}", path));
    }

    // Read the file
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Check content size
    if content.len() > MAX_CONTENT_SIZE {
        return Err("Document content exceeds maximum size limit (10MB)".to_string());
    }

    // Log the operation
    // log_file_operation("read", "user", &file_path, true, None); // TODO: Implement

    println!("Document loaded successfully from: {}", path);
    Ok(content)
}

async fn validate_path(path: String) -> Result<(), String> {
    // Check for path traversal patterns
    let malicious_patterns = [
        "..",
        "../",
        "..\\",
        "..%2F",
        "..%5C",
        "../..",
        "../..\\",
        "....//",
        "....\\\\",
        "\\\\evil-server",
        "/etc/passwd",
        "C:\\Windows\\System32",
        "file://",
        "file:",
    ];

    let path_lower = path.to_lowercase();

    for pattern in &malicious_patterns {
        if path_lower.contains(pattern) {
            return Err(format!("Path traversal attempt detected: {}", path));
        }
    }

    // Additional validation for absolute paths that could be dangerous
    if path.starts_with("/etc/") || path.starts_with("C:\\Windows\\") || path.contains("system32") {
        return Err(format!("Potentially dangerous path detected: {}", path));
    }

    Ok(())
}

/// Import project implementation - copy files from source to destination
pub async fn import_project_impl(source_path: String, dest_path: String) -> Result<(), String> {
    // Validate both paths for security
    validate_path(source_path.clone()).await?;
    validate_path(dest_path.clone()).await?;

    let source = Path::new(&source_path);
    let dest = Path::new(&dest_path);

    if !source.exists() {
        return Err(format!("Source path does not exist: {}", source_path));
    }

    if source.is_dir() {
        // If source is a directory, copy the entire directory
        copy_directory(source, dest).map_err(|e| format!("Failed to copy directory: {}", e))?;
    } else {
        // If source is a file, copy the single file
        copy_file(source, dest).map_err(|e| format!("Failed to copy file: {}", e))?;
    }

    // Log the operation
    // log_file_operation("import", "user", dest, true, None); // TODO: Implement

    println!(
        "Project imported successfully from {} to {}",
        source_path, dest_path
    );
    Ok(())
}

fn copy_file(source: &Path, dest: &Path) -> Result<(), std::io::Error> {
    // Ensure destination directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy the file
    fs::copy(source, dest)?;
    Ok(())
}

fn copy_directory(source: &Path, dest: &Path) -> Result<(), std::io::Error> {
    // Ensure destination directory exists
    fs::create_dir_all(dest)?;

    // Read source directory entries
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            // Recursively copy subdirectories
            copy_directory(&entry.path(), &dest_path)?;
        } else if file_type.is_file() {
            // Copy files
            copy_file(&entry.path(), &dest_path)?;
        }
        // Skip symlinks and other special file types for safety
    }

    Ok(())
}
