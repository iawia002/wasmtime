//! The module that implements the `wasmtime wast` command.

use anyhow::{Context as _, Result};
use clap::Parser;
use std::path::PathBuf;
use wasmtime::{Engine, Store};
use wasmtime_cli_flags::CommonOptions;
use wasmtime_wast::WastContext;

/// Runs a WebAssembly test script file
#[derive(Parser)]
#[clap(name = "wast", version)]
pub struct WastCommand {
    #[clap(flatten)]
    common: CommonOptions,

    /// The path of the WebAssembly test script to run
    #[clap(required = true, value_name = "SCRIPT_FILE")]
    scripts: Vec<PathBuf>,
}

impl WastCommand {
    /// Executes the command.
    pub fn execute(mut self) -> Result<()> {
        self.common.init_logging()?;

        let config = self.common.config(None)?;
        let store = Store::new(&Engine::new(&config)?, ());
        let mut wast_context = WastContext::new(store);

        wast_context
            .register_spectest(true)
            .expect("error instantiating \"spectest\"");

        for script in self.scripts.iter() {
            wast_context
                .run_file(script)
                .with_context(|| format!("failed to run script file '{}'", script.display()))?
        }

        Ok(())
    }
}
