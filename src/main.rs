use anyhow::Result;

mod roll;
mod dice;
mod app;

fn main() -> Result<()> {
    app::run()
}