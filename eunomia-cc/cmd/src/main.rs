mod compile_bpf;
mod config;
mod document_parser;
mod export_types;

use anyhow::Result;
use clap::Parser;
use compile_bpf::*;
use config::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    compile_bpf(&args)?;
    if !args.subskeleton {
        pack_object_in_config(&args)?;
    }
    Ok(())
}
