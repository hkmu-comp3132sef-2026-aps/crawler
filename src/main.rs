pub mod configs;
pub mod consts;
pub mod fetch;
pub mod functions;
pub mod sql;
pub mod structs;

use dotenv_plus::DotEnv;

use crate::fetch::fetch_schools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    DotEnv::new().run();

    fetch_schools().await
}
