// treecat.rs.d/src/main.rs

mod app;
mod classify;
mod render;
mod repo;
mod util;
mod walk;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}