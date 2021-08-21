use std::time::Duration;

use tokio::time::sleep;
use watchexec::{
	config::{InitConfigBuilder, RuntimeConfigBuilder},
	Watchexec,
};

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
	tracing_subscriber::fmt::init();
	color_eyre::install()?;

	let init = InitConfigBuilder::default()
		.error_handler(Box::new(|err| async move {
			eprintln!("Watchexec Runtime Error: {}", err);
			Ok::<(), std::convert::Infallible>(())
		}))
		.build()?;

	let runtime = RuntimeConfigBuilder::default().build()?;

	let wx = Watchexec::new(init, runtime)?;
	wx.main();

	// TODO: induce an error here

	sleep(Duration::from_secs(1)).await;

	Ok(())
}
