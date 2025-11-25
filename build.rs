//! Build script for Herding Cats Rust - Slint Integration
//! This script handles Slint compilation and code generation

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to rerun this build script if any .slint files change
    println!("cargo:rerun-if-changed=src/ui/*.slint");
    println!("cargo:rerun-if-changed=src/ui/**/*.slint");
    println!("cargo:rerun-if-changed=src/**/*.slint");

    // Tell cargo to invalidate the built crate whenever the build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Find all .slint files in the project
    let slint_files = find_slint_files("src")?;

    if !slint_files.is_empty() {
        println!(
            "cargo:warning=Found {} Slint files to compile",
            slint_files.len()
        );

        // Print the files that will be processed
        for file in &slint_files {
            println!("cargo:warning=Processing: {}", file.display());
        }

        // For now, just inform that Slint files exist
        // The actual compilation will be handled by the slint crate
        println!(
            "cargo:warning=Slint files detected - compilation will be handled by slint macros"
        );
    } else {
        println!("cargo:warning=No Slint files found to compile");
    }



    // Compile the Font Manager UI
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".into());
    
    slint_build::compile_with_config(
        "src/ui/all_modules.slint",
        config,
    )?;

    Ok(())
}

fn find_slint_files(dir: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_slint_files(&path.to_string_lossy())?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("slint") {
                files.push(path);
            }
        }
    }
    Ok(files)
}
