mod rattler_build;

use rattler_build::RattlerBuildBackend;

#[tokio::main]
pub async fn main() {
    if let Err(err) = pixi_build_backend::cli::main(RattlerBuildBackend::factory).await {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
