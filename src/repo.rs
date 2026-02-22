// treecat.rs.d/src/repo.rs

use anyhow::Result;
use git2::Repository;
use std::env;
use std::path::{Component, Path, PathBuf};

pub fn discover_repo_root() -> Result<(Option<PathBuf>, bool)> {
    let cwd = env::current_dir()?;
    match Repository::discover(&cwd) {
        Ok(repo) => Ok((repo.workdir().map(Path::to_path_buf), true)),
        Err(_) => Ok((None, false)),
    }
}

pub fn normalize_git_args(repo_root: &Path, args: &[PathBuf]) -> Vec<PathBuf> {
    let mut rels = Vec::new();

    for arg in args {
        // If user passed "." (or any relative), keep it relative. This avoids path mismatches.
        let rel = if arg.is_absolute() {
            if arg == repo_root {
                PathBuf::from(".")
            } else if is_within(repo_root, arg) {
                make_relative(repo_root, arg)
            } else {
                eprintln!("treecat: skipping path outside repo: {}", arg.display());
                continue;
            }
        } else {
            arg.clone()
        };

        rels.push(rel);
    }

    rels
}

fn is_within(root: &Path, p: &Path) -> bool {
    let mut root_comps = root.components();
    let mut p_comps = p.components();

    loop {
        match (root_comps.next(), p_comps.next()) {
            (None, _) => return true,
            (Some(r), Some(pc)) if r == pc => continue,
            _ => return false,
        }
    }
}

fn make_relative(root: &Path, p: &Path) -> PathBuf {
    let mut rel = PathBuf::new();
    let mut root_iter = root.components().peekable();
    let mut p_iter = p.components().peekable();

    while let (Some(r), Some(pp)) = (root_iter.peek(), p_iter.peek()) {
        if r == pp {
            root_iter.next();
            p_iter.next();
        } else {
            break;
        }
    }

    for comp in p_iter {
        if matches!(comp, Component::Prefix(_) | Component::RootDir) {
            continue;
        }
        rel.push(comp.as_os_str());
    }

    if rel.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        rel
    }
}
