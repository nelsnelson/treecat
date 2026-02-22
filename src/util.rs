// treecat.rs.d/src/util.rs

use std::ffi::OsStr;
use std::path::Path;

pub fn is_noise_excluded(path: &Path) -> bool {
    let name = path.file_name().and_then(OsStr::to_str).unwrap_or("");
    name == ".DS_Store" || name.starts_with("._")
}

pub fn contains_dot_git(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == OsStr::new(".git"))
}
