use anyhow::Result;
use crate::main_window_comprehensive::MainWindowComprehensive;

pub async fn run_working_app() -> Result<()> {
    let window = MainWindowComprehensive::new().await?;

    window.run()?;

    Ok(())
}
