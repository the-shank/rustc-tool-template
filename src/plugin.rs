use std::env;
use std::process::Command;

use camino::Utf8Path;
use clap::Parser;
use color_eyre::eyre::{self, Context as _};
use rustc_driver::{Compilation, TimePassesCallbacks};
use rustc_interface::Queries;
use rustc_plugin::{CrateFilter, RustcPlugin, RustcPluginArgs};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::analysis::{analyze, process_result, AnalysisConfig, AnalysisResult};
use crate::utils;

#[derive(Debug, Clone, Copy, Default)]
pub struct CompilerPlugin;

#[derive(Debug, Parser, Clone, Serialize, Deserialize)]
pub struct PluginArgs {
    // TODO: add other args as required here

    // use cargo_args as the last, to pass args to cargo
    #[clap(last = true)]
    cargo_args: Vec<String>,
}

impl RustcPlugin for CompilerPlugin {
    type Args = PluginArgs;

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "tool-driver".into()
    }

    fn args(&self, _target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args> {
        let args = PluginArgs::parse_from(env::args().skip(1));
        let filter = CrateFilter::OnlyWorkspace;
        RustcPluginArgs { args, filter }
    }

    // Pass Cargo arguments (like --feature) from the top-level CLI to Cargo.
    fn modify_cargo(&self, cargo: &mut Command, args: &Self::Args) {
        cargo.args(&args.cargo_args);
    }

    fn run(
        self,
        compiler_args: Vec<String>,
        plugin_args: Self::Args,
    ) -> rustc_interface::interface::Result<()> {
        let crate_type = utils::get_crate_type(&compiler_args);

        if !crate_type
            .as_ref()
            .is_some_and(|typ| utils::is_lib_type(typ))
        {
            // we need to build the crate normally here, using cargo.
            // Otherwise the build scripts might not be executed.
            debug!(">> building normally as crate-type is not a lib type");
            let mut callbacks = TimePassesCallbacks::default();
            let compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
            return compiler.run();
        }

        let mut compiler_args = compiler_args;
        compiler_args.push("-Awarnings".to_string());
        compiler_args.push("-Zno-codegen".to_string());
        compiler_args.push("-Zmir-opt-level=0".to_string());

        let mut callbacks = Callbacks::new(plugin_args);

        let compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
        debug!(">> starting compiler.run() ...");
        compiler.run()?;

        let analysis_result = callbacks
            .analysis_result
            .as_ref()
            .expect("analysis_result is None");
        process_result(&analysis_result)
            .wrap_err_with(|| {
                eyre::eyre!("failed while processing analysis_result [PLEASE INVESTIGATE]")
            })
            .unwrap();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Callbacks {
    analysis_config: AnalysisConfig,
    analysis_result: Option<AnalysisResult>,
}

impl Callbacks {
    pub fn new(args: PluginArgs) -> Self {
        Self {
            analysis_config: AnalysisConfig::from(args),
            analysis_result: None,
        }
    }
}

impl rustc_driver::Callbacks for Callbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        let res = queries
            .global_ctxt()
            .unwrap()
            .enter(|tcx| analyze(tcx, &self.analysis_config));

        match res {
            Ok(analysis_result) => {
                self.analysis_result = Some(analysis_result);
                Compilation::Continue
            }
            Err(e) => {
                error!("failed during analysis [PLEASE INVESTIGATE]\n{e:?}");
                Compilation::Stop
            }
        }
    }
}
