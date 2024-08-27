mod kessler;
use crate::kessler::Kessler;
// pub mod repo;

use clap::Parser;
use color_eyre::eyre::Result;
use stardust_xr_fusion::{client::Client, node::NodeType, root::RootAspect};
use std::path::PathBuf;

#[derive(Clone, Parser)]
pub struct Args {
    pub repo_path: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // console_subscriber::init();
    let args = Args::parse();
    color_eyre::install().unwrap();
    let (client, event_loop) = Client::connect_with_async_loop()
        .await
        .expect("Couldn't connect");
    client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

    let _root = client
        .get_root()
        .alias()
        .wrap(Kessler::new(&client, args)?)?;

    tokio::select! {
        biased;
        _ = tokio::signal::ctrl_c() => Ok(()),
        e = event_loop => e?.map_err(|e| e.into()),
    }
}
