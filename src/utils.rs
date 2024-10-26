use std::{path, process};

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

pub fn get_crate_names() -> eyre::Result<Vec<String>> {
    let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec()?;

    let mut cratenames = Vec::new();
    for pkg in &metadata.packages {
        for target in &pkg.targets {
            if target.kind.iter().any(|k| k.contains("lib")) {
                cratenames.push(pkg.name.clone());
            }
        }
    }

    Ok(cratenames)
}

pub fn get_crate_name() -> eyre::Result<String> {
    let cratenames = get_crate_names()?;

    eyre::ensure!(
        !cratenames.is_empty(),
        "no cratename found for lib targets [PLEASE INVESTIGATE]"
    );
    eyre::ensure!(
        cratenames.len() == 1,
        "multiple cratenames found that have lib targets [PLEASE INVESTIGATE]"
    );

    Ok(cratenames.into_iter().next().unwrap())
}

pub fn clean_crate_by_name(name: &str) -> eyre::Result<()> {
    process::Command::new("cargo")
        .arg("clean")
        .arg("-p")
        .arg(name)
        .status()
        .wrap_err_with(|| String::from("cargo clean failed"))?;
    Ok(())
}
