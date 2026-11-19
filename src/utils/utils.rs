use std::path::PathBuf;

pub fn get_path() -> Result<PathBuf, anyhow::Error> {
    let exe_path = std::env::current_exe()?;
    let real_exe = std::fs::canonicalize(&exe_path).unwrap_or(exe_path);

    let exe_dir = real_exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Unable to get the binary folder"))?;

    let server_root = exe_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Unable to move up one level from the binary folder"))?;

    Ok(server_root.to_path_buf())
}