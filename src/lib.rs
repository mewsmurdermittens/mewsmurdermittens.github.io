//! mewsmurdermittens static site generator
//!
//! #FIXME - more doc

#![forbid(unsafe_code)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![warn(clippy::unwrap_used)]

use entrypoint::prelude::*;
use std::path::PathBuf;

/// CLI options
#[derive(clap::Parser, DotEnvDefault, LoggerDefault, Debug)]
#[log_level(entrypoint::tracing::Level::DEBUG)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    /// directory to save generated output
    #[clap(long, env)]
    output_directory: Option<PathBuf>,
}

/// required config
#[derive(Debug)]
pub struct Config {
    /// directory to save generated output
    output_directory: PathBuf,
}

impl TryFrom<CLIArgs> for Config {
    type Error = entrypoint::anyhow::Error;

    fn try_from(item: CLIArgs) -> entrypoint::anyhow::Result<Self> {
        let output_directory: PathBuf = if let Ok(dir) = std::env::var("OUTPUT_DIRECTORY") {
            dir.into()
        } else {
            item.output_directory
                .context("no output_directory supplied")?
        };

        Ok(Self { output_directory })
    }
}

/// #FIXME - doc
pub fn run(config: Config) -> entrypoint::anyhow::Result<()> {
    info!("{:#?}", config);
    Ok(())
}
