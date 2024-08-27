// mod kessler;
// pub mod repo;

use asteroids::{ElementTrait, Model, Spatial, StardustClient};
use clap::Parser;
use manifest_dir_macros::directory_relative_path;
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::client::Client;
use std::path::PathBuf;

#[derive(Clone, Parser)]
pub struct Args {
    pub repo_path: PathBuf,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct State {
    repo_path: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // console_subscriber::init();
    color_eyre::install().unwrap();
    let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
    client
        .set_base_prefixes(&[directory_relative_path!("res")])
        .unwrap();

    let _asteroids = StardustClient::new(
        client,
        || {
            let args = Args::parse();
            State {
                repo_path: args.repo_path,
            }
        },
        |state, frame_info| {},
        |state| {
            Spatial::default().zoneable(true).with_children([
                // Model::namespaced("asteroids", "grabbable").build(),
                Model::namespaced("kessler", "pot").build(),
            ])
        },
    )
    .unwrap();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => (),
        _ = event_loop => panic!("server crashed"),
    }
}
