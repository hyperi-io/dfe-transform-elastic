// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Binary entry point.

use clap::Parser;
use dfe_transform_elastic::cli::App;

// Every binary gets a global allocator; hyperi-ci enables this feature on
// spike/alpha/beta/release. Local dev stays on the system allocator.
#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let app = App::parse();

    if let Err(e) = scalo::cli::run_app(app).await {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}
