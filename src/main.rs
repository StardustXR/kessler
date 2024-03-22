mod kessler;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	// console_subscriber::init();
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");


	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => Ok(()),
		e = event_loop => e?.map_err(|e| e.into()),
	}
}

struct Kessler {
	root: Spatial,
}
impl RootHandler for Kessler {
	fn frame(&mut self, _info: FrameInfo) {
	}
	fn save_state(&mut self) -> ClientState {
		ClientState::from_root(&self.1)
	}
}
