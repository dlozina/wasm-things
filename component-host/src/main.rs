mod hello_component;
use clap::Parser;
use std::path::PathBuf;

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-component", version = env!("CARGO_PKG_VERSION"))]
struct GetMessage {
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

impl GetMessage {
    async fn run(self) -> anyhow::Result<()> {
        let message = hello_component::get_message(self.component).await?;
        println!("Here is the message from WASM component: {message}");
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    GetMessage::parse().run().await
}