// treecat.rs.d/src/walk.rs

use crate::util::{contains_dot_git, is_noise_excluded};
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub fn collect_files_git(repo_root: &Path, rels: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut builder = WalkBuilder::new(repo_root);
    builder
        .hidden(false)
        .ignore(false)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .filter_entry(|e| !contains_dot_git(e.path()));

    for rel in rels {
        builder.add(rel);
    }

    let mut files = Vec::new();
    for entry in builder.build() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let p = entry.into_path();
        if is_noise_excluded(&p) {
            continue;
        }

        files.push(p);
    }

    Ok(files)
}

pub fn collect_files_nongit(args: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for root in args {
        let mut builder = WalkBuilder::new(root);
        builder
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .git_exclude(false)
            .git_global(false)
            .filter_entry(|e| !contains_dot_git(e.path()));

        for entry in builder.build() {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                continue;
            }

            let p = entry.into_path();
            if is_noise_excluded(&p) {
                continue;
            }

            files.push(p);
        }
    }

    Ok(files)
}
