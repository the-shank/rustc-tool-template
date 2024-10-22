use std::path;

use color_eyre::eyre::{self, Context as _};

pub fn parse_dir(s: &str) -> eyre::Result<path::PathBuf> {
    path::Path::new(s)
        .try_exists()
        .wrap_err_with(|| format!("{s} isn't a valid directory"))?;
    Ok(s.into())
}
