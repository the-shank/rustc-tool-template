#![deny(absolute_paths_not_starting_with_crate)]
#![deny(deprecated_in_future)]
#![deny(elided_lifetimes_in_paths)]
#![deny(explicit_outlives_requirements)]
#![deny(keyword_idents)]
#![deny(macro_use_extern_crate)]
#![deny(meta_variable_misuse)]
#![deny(missing_abi)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(non_ascii_idents)]
#![deny(noop_method_call)]
#![deny(rust_2021_incompatible_closure_captures)]
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(rust_2021_prefixes_incompatible_syntax)]
#![deny(rust_2021_prelude_collisions)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
// #![deny(warnings)]
#![feature(rustc_private)]

use std::fs::File;
use std::io::{self, IsTerminal as _};
use std::path::PathBuf;
use std::time::Instant;

use analysis::AnalysisConfig;
use clap::Parser;
use color_eyre::eyre::{self, Context as _};
use tool::utils::parse_dir;
use tool::{analysis, transform};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
struct Args {
    /// path to the source dir for the crate to be anlayzed
    #[arg(long, value_parser=parse_dir)]
    cratedir: PathBuf,

    #[arg(long)]
    dump_analysis_result: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    transform: bool,
}

fn main() -> eyre::Result<()> {
    let start = Instant::now();

    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .with_ansi(io::stdout().is_terminal())
        .init();

    let args = Args::parse();
    info!("args: {:#?}", &args);

    eyre::ensure!(args.cratedir.is_dir());

    let conf = AnalysisConfig;
    let analysis_result = analysis::analyze_path(&args.cratedir, &conf)?;

    if let Some(dump_file) = args.dump_analysis_result {
        let f = File::create(&dump_file).unwrap();
        serde_json::to_writer_pretty(f, &analysis_result)
            .wrap_err_with(|| format!("failed to write the results to {}", dump_file.display()))?;
    }

    if args.transform {
        transform::transform_path(&args.cratedir, &analysis_result)?;
    }

    println!("Total Time : {:.3} seconds", start.elapsed().as_secs_f32());
    Ok(())
}
