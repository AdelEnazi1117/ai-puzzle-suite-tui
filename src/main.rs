mod app;
mod puzzles;
mod search;
mod ui;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut application = app::App::default();
    ui::run(&mut application)
}
