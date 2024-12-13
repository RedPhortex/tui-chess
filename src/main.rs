use color_eyre::Result;

/// Application.
mod app;
pub use app::App;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

// Utils methods and types.
pub mod utils;

// Widgets.
pub mod widgets;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;

    ratatui::restore();
    Ok(app_result?)
}
