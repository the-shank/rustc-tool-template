use std::path::Path;

use color_eyre::eyre;

use crate::analysis::AnalysisResult;

pub fn transform_path(path: &Path, params: &AnalysisResult) -> eyre::Result<()> {
    // TODO: this path should really be the output cratedir or the input cratedir?? 🤔
    // TODO: Write your code rewriting/transformation here.
    todo!()
}
