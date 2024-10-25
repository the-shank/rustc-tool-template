use std::path;

use color_eyre::eyre::{self, Context as _};

pub fn parse_dir(s: &str) -> eyre::Result<path::PathBuf> {
    path::Path::new(s)
        .try_exists()
        .wrap_err_with(|| format!("{s} isn't a valid directory"))?;
    Ok(s.into())
}

pub(crate) fn get_crate_type(compiler_args: &[String]) -> Option<&str> {
    let mut return_this = false;

    for arg in compiler_args {
        if return_this {
            return Some(arg);
        }
        if arg.as_str() == "--crate-type" {
            return_this = true;
        }
    }

    None
}

pub(crate) fn is_lib_type(typ: &str) -> bool {
    typ == "lib" || typ == "rlib" || typ == "dylib" || typ == "cdylib" || typ == "staticlib"
}
