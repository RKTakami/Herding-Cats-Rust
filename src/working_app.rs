use anyhow::Result;

pub async fn run_working_app() -> Result<()> {
    let window = crate::main_window_comprehensive::MainWindowComprehensive::new().await?;

    window.run()?;

    Ok(())
}
