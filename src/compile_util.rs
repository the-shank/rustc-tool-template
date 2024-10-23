use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use rustc_errors::registry::Registry;
use rustc_feature::UnstableFeatures;
use rustc_hash::FxHashMap;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{self, CrateType, Input};
use rustc_span::edition::Edition;
use rustc_span::FileName;

pub fn str_to_input(code: &str) -> Input {
    Input::Str {
        name: FileName::Custom("main.rs".to_string()),
        input: code.to_string(),
    }
}

pub fn path_to_input(path: &Path) -> Input {
    Input::File(path.to_path_buf())
}

pub fn make_config(input: Input) -> rustc_interface::Config {
    rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot())),
            unstable_features: UnstableFeatures::Allow,
            crate_types: vec![CrateType::Rlib],
            // FIXME: edition should not be fixed...
            edition: Edition::Edition2021,
            unstable_opts: config::UnstableOptions {
                mir_opt_level: Some(0),
                ..config::UnstableOptions::default()
            },
            ..config::Options::default()
        },
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        input,
        output_dir: None,
        output_file: None,
        ice_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES.to_vec(),
        lint_caps: FxHashMap::default(),
        psess_created: None,
        hash_untracked_state: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: Registry::new(rustc_errors::codes::DIAGNOSTICS),
        using_internal_features: Arc::default(),
        expanded_args: Vec::new(),
    }
}

fn sysroot() -> String {
    std::env::var("SYSROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let home = std::env::var("RUSTUP_HOME")
                .or_else(|_| std::env::var("MULTIRUST_HOME"))
                .ok();
            let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
                .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
                .ok();
            toolchain_path(home, toolchain)
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME")
                .or(option_env!("MULTIRUST_HOME"))
                .map(ToString::to_string);
            let toolchain = option_env!("RUSTUP_TOOLCHAIN")
                .or(option_env!("MULTIRUST_TOOLCHAIN"))
                .map(ToString::to_string);
            toolchain_path(home, toolchain)
        })
        .map(|pb| pb.to_string_lossy().to_string())
        .expect("we would always get *some* sysroot...")
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
    home.and_then(|home| {
        toolchain.map(|toolchain| {
            let mut path = PathBuf::from(home);
            path.push("toolchains");
            path.push(toolchain);
            path
        })
    })
}

pub fn run_compiler<R, F>(config: rustc_interface::Config, func: F) -> R
where
    R: Send,
    F: FnOnce(TyCtxt<'_>) -> R + Send,
{
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries
                .global_ctxt()
                .ok()
                .expect("failed to get global_ctxt")
                .enter(|tcx| func(tcx))
        })
    })
}
