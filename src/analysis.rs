use color_eyre::eyre;
use rustc_middle::ty::TyCtxt;
use serde::{Deserialize, Serialize};

use crate::plugin::PluginArgs;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig;

impl AnalysisConfig {
    pub(crate) fn from(plugin_args: PluginArgs) -> Self {
        Self
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult;

// The main analysis function
pub(crate) fn analyze(tcx: TyCtxt<'_>, conf: &AnalysisConfig) -> eyre::Result<AnalysisResult> {
    // TODO: Write your analysis here.
    todo!("implement analyze()")
}

pub(crate) fn process_result(analysis_result: &AnalysisResult) -> eyre::Result<()> {
    // TODO: Process the result here
    todo!("implement process_result()")
}
