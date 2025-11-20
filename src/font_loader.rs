//! Font loading module for Slint integration
//! Provides dynamic font loading capabilities for the UI

use fontdb::Database;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global font database and cache
static FONT_DB: Lazy<Mutex<Database>> = Lazy::new(|| {
    let mut db = Database::new();
    // Load system fonts initially
    db.load_system_fonts();
    Mutex::new(db)
});

/// Cache of loaded font families
static LOADED_FONTS: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Initialize the font loader
pub fn init_font_loader() -> Result<(), String> {
    println!("=== Initializing font loader ===");
    
    println!("Font database initialization...");
    let mut db = FONT_DB.lock().map_err(|e| format!("Failed to lock font database during init: {}", e))?;
    
    println!("Font database initialized successfully");
    
    // Try to load system fonts explicitly
    println!("Loading system fonts...");
    db.load_system_fonts();
    
    let face_count = db.faces().count();
    println!("Initial font face count: {}", face_count);
    
    // Count families
    let mut families = std::collections::HashSet::new();
    for face in db.faces() {
        if let Some(family) = face.families.first() {
            families.insert(family.0.clone());
        }
    }
    println!("Initial font family count: {}", families.len());
    println!("Initial font families: {:?}", families);
    
    // Only use system fonts - no fallbacks
    if families.is_empty() {
        println!("WARNING: No system fonts detected by fontdb");
    }
    
    println!("Font loader initialization complete");
    println!("=== Font loader initialized ===");
    
    Ok(())
}

/// Load a font from file path and make it available to Slint
pub fn load_font_from_path<P: AsRef<Path>>(font_path: P, family_name: &str) -> Result<(), String> {
    let font_path = font_path.as_ref();

    if !font_path.exists() {
        return Err(format!("Font file does not exist: {:?}", font_path));
    }

    // Read the font file
    let font_data = std::fs::read(font_path)
        .map_err(|e| format!("Failed to read font file: {}", e))?;

    load_font_from_memory(&font_data, family_name)
}

/// Load a font from memory buffer and make it available to Slint
pub fn load_font_from_memory(font_data: &[u8], family_name: &str) -> Result<(), String> {
    let mut db = FONT_DB.lock().map_err(|e| format!("Failed to lock font database: {}", e))?;

    // Try to load the font into the database
    db.load_font_data(font_data.to_vec());

    println!("Font '{}' loaded into database", family_name);

    // Store in our cache for potential future use
    let mut loaded_fonts = LOADED_FONTS.lock().map_err(|e| format!("Failed to lock loaded fonts: {}", e))?;
    loaded_fonts.insert(family_name.to_string(), font_data.to_vec());

    Ok(())
}

/// Check if a font family is loaded
pub fn is_font_loaded(family_name: &str) -> bool {
    let loaded_fonts = match LOADED_FONTS.lock() {
        Ok(loaded) => loaded,
        Err(_) => return false,
    };

    loaded_fonts.contains_key(family_name)
}

/// Get information about loaded fonts
pub fn get_loaded_fonts() -> Vec<String> {
    let loaded_fonts = match LOADED_FONTS.lock() {
        Ok(loaded) => loaded,
        Err(_) => return Vec::new(),
    };

    loaded_fonts.keys().cloned().collect()
}

/// Query the font database for available fonts
pub fn query_font_database(query: &str) -> Vec<String> {
    println!("=== query_font_database called ===");
    println!("Query parameter: '{}'", query);
    
    let db = match FONT_DB.lock() {
        Ok(db) => {
            println!("Font database lock acquired successfully");
            db
        },
        Err(e) => {
            println!("ERROR: Failed to lock font database: {}", e);
            return Vec::new();
        },
    };

    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    // Query faces in the database - ONLY system fonts
    println!("Iterating through font faces...");
    let mut face_count = 0;
    for face in db.faces() {
        face_count += 1;
        println!("Processing face #{}: {:?}", face_count, face);
        
        if let Some(family) = face.families.first() {
            let family_name = family.0.clone();
            println!("Found system family: {} (query: '{}')", family_name, query_lower);
            
            // If query is empty, include all families. Otherwise, filter by query.
            if query.is_empty() || family_name.to_lowercase().contains(&query_lower) {
                if !results.contains(&family_name) {
                    println!("Adding system family to results: {}", family_name);
                    results.push(family_name);
                } else {
                    println!("Family already in results (duplicate): {}", family_name);
                }
            } else {
                println!("Family doesn't match query filter: {}", family_name);
            }
        } else {
            println!("Face has no families: {:?}", face);
        }
    }
    
    println!("Total faces processed: {}", face_count);
    println!("System fonts found: {:?}", results);

    results.sort();
    results.dedup();
    println!("Final system fonts count: {}", results.len());
    println!("Final system fonts: {:?}", results);
    println!("=== query_font_database complete - SYSTEM FONTS ONLY ===");
    results
}

/// Get font database statistics
pub fn get_font_stats() -> (usize, usize, usize) {
    let db = match FONT_DB.lock() {
        Ok(db) => db,
        Err(_) => return (0, 0, 0),
    };

    let face_count = db.faces().count();

    // Count unique families
    let mut families = std::collections::HashSet::new();
    for face in db.faces() {
        if let Some(family) = face.families.first() {
            families.insert(family.0.clone());
        }
    }
    let family_count = families.len();

    let loaded_fonts = match LOADED_FONTS.lock() {
        Ok(loaded) => loaded.len(),
        Err(_) => 0,
    };

    (face_count, family_count, loaded_fonts)
}

/// Clean up loaded fonts (useful for testing or memory management)
pub fn unload_font(family_name: &str) -> Result<(), String> {
    let mut loaded_fonts = LOADED_FONTS.lock().map_err(|e| format!("Failed to lock loaded fonts: {}", e))?;

    if loaded_fonts.remove(family_name).is_some() {
        println!("Font '{}' unloaded from cache", family_name);
        Ok(())
    } else {
        Err(format!("Font '{}' was not loaded", family_name))
    }
}