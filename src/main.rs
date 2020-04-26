#![deny(missing_docs, missing_debug_implementations, unused_results)]

//! MCPacker helps maintain Minecarft mod packs.

mod app;
mod errors;
mod manifest;
mod manifest_json;
mod twitch_api;

use app::App;
use errors::Result;

fn main() -> Result<()> {
    App::run()
}
