mod app;
mod errors;
mod manifest;
mod manifest_json;
mod options_sorted;

use app::App;
use errors::Result;

fn main() -> Result<()> {
    App::run()
}
