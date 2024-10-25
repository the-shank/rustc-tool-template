# Rustc-tool Template

Template for writing rust anlaysis/rewriting tools.

## How to use

- Define your analysis config in `src/analysis.rs` -> `AnalysisConfig`.
- Define your analysis in `src/analysis.rs` -> `analyze()`.
- Define your analysis results in `src/analysis.rs` -> `AnalysisResult`.

### Credits

- This template is based on [RustcPlugin](https://github.com/cognitive-engineering-lab/rustc_plugin)
