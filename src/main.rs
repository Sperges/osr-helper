use anyhow::Result;

mod roll;
mod dice;
mod app;
mod table;

fn main() -> Result<()> {
    app::run()
}