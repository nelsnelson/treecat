// treecat.rs.d/src/app.rs

use crate::render::write_one;
use crate::repo::{discover_repo_root, normalize_git_args};
use crate::walk::{collect_files_git, collect_files_nongit};
use anyhow::Result;
use infer::Infer;
use rayon::prelude::*;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn run() -> Result<()> {
    // Match the bash script: no args defaults to "." (not $PWD expanded).
    let mut args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    if args.is_empty() {
        args.push(PathBuf::from("."));
    }

    let (repo_root, is_git) = discover_repo_root()?;

    let files = if is_git {
        let repo_root = repo_root.expect("repo_root present if is_git");
        let rels = normalize_git_args(&repo_root, &args);
        collect_files_git(&repo_root, &rels)?
    } else {
        collect_files_nongit(&args)?
    };

    if files.is_empty() {
        return Ok(());
    }

    // Best-effort ordering: do not sort; walk order + completion order wins.

    let infer = Infer::new();

    // Single global output token to prevent interleaving file blocks.
    let out_lock = Arc::new(Mutex::new(()));

    files.par_iter().try_for_each(|path| -> Result<()> {
        let _guard = out_lock.lock().expect("output lock poisoned");
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        write_one(&infer, path, &mut stdout)?;
        stdout.flush()?;
        Ok(())
    })?;

    Ok(())
}
