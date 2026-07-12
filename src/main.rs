mod platform;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run()?;
    Ok(())
}
