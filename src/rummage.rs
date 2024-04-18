use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn edit<P: AsRef<Path>>(path: P) -> Result<(), &'static str> {
    let (editor, args) = get_editor_args()?;
    let status = Command::new(editor)
        .args(args)
        .arg(path.as_ref())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .or_else(|_| Err("Failed to execute editor executable"))?
        .status;

    if status.success() {
        Ok(())
    } else {
        Err("Something went wrong...")
    }
}

static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];

fn get_editor_args() -> Result<(PathBuf, Vec<String>), &'static str> {
    ENV_VARS
        .iter()
        .filter_map(env::var_os)
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.into_string().ok())
        .filter_map(|s| get_full_editor_cmd(s).ok())
        .next()
        .ok_or_else(|| "Could not find path to default editor executable. Configure editor executable (SEE README)")
}

/// converts env var into executable name as PathBuf, and additional args
fn string_to_cmd(s: String) -> (PathBuf, Vec<String>) {
    let mut args = s.split_ascii_whitespace();
    (
        args.next().unwrap().into(),
        args.map(String::from).collect(),
    )
}

/// tries first to locate the executable path, then falls back to 'raw name', else fails
fn get_full_editor_cmd(s: String) -> Result<(PathBuf, Vec<String>), &'static str> {
    let (path, args) = string_to_cmd(s);
    match get_full_editor_path(&path) {
        Ok(result) => Ok((result, args)),
        Err(_) if path.exists() => Ok((path, args)),
        Err(_) => Err(
            "Could not find path to editor executable. Configure editor executable (SEE README)",
        ),
    }
}

/// tries to get editor executable from $PATH
fn get_full_editor_path<T: AsRef<OsStr> + AsRef<Path>>(
    binary_name: T,
) -> Result<PathBuf, &'static str> {
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            if dir.join(&binary_name).is_file() {
                return Ok(dir.join(&binary_name));
            }
        }
    }

    // todo: Further error msg specifics for config, extract this error message to constant
    Err("Could not find path to editor executable. Configure editor executable (SEE README)")
}
