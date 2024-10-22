use std::path::Path;

use color_eyre::eyre;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::Input;
use serde::{Deserialize, Serialize};

use crate::compile_util;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult;

pub fn analyze_path(path: &Path, conf: &AnalysisConfig) -> eyre::Result<AnalysisResult> {
    analyze_input(compile_util::path_to_input(path), conf)
}

pub fn analyze_code(code: &str, conf: &AnalysisConfig) -> eyre::Result<AnalysisResult> {
    analyze_input(compile_util::str_to_input(code), conf)
}

pub fn analyze_input(input: Input, conf: &AnalysisConfig) -> eyre::Result<AnalysisResult> {
    let compiler_config = compile_util::make_config(input);
    compile_util::run_compiler(compiler_config, |tcx| analyze(tcx, conf))
}

// The main analysis function
fn analyze(tcx: TyCtxt<'_>, conf: &AnalysisConfig) -> eyre::Result<AnalysisResult> {
    // TODO: Write your analysis here.
    todo!()
}
