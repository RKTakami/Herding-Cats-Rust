use anyhow::Result;
use herding_cats_rust as hc_lib;
use hc_lib::main_window_comprehensive::MainWindowComprehensive;

pub async fn run_working_app() -> Result<()> {
    let window = MainWindowComprehensive::new().await?;

    window.run()?;

    // Graceful shutdown
    window.shutdown().await;

    Ok(())
}
