#![deny(missing_docs, missing_debug_implementations, unused_results)]

//! MCPacker helps maintain Minecarft mod packs.

mod app;
mod errors;
mod manifest;
mod manifest_json;

use app::App;
use errors::Result;

fn main() -> Result<()> {
    App::run()
}
