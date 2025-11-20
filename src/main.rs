//! Herding Cats Rust - Slint Multi-Window Application
//! Modern UI framework with Slint

mod ui;
mod working_app;

/// Main application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the working app as the main entry point
    working_app::run_working_app().await?;
    Ok(())
}
