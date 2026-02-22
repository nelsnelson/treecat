// treecat.rs.d/src/render.rs

use crate::classify::{looks_like_text, SAMPLE_BYTES};
use anyhow::{Context, Result};
use infer::Infer;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub fn write_one(infer: &Infer, path: &Path, mut w: impl Write) -> Result<()> {
    writeln!(w, "{}:", path.display()).context("write header")?;

    let mut f = fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let meta = f.metadata().with_context(|| format!("stat {}", path.display()))?;
    let size = meta.len();

    let mut sample = vec![0u8; SAMPLE_BYTES.min(size as usize)];
    if !sample.is_empty() {
        f.read_exact(&mut sample)
            .with_context(|| format!("read sample {}", path.display()))?;
    }

    if looks_like_text(&sample) {
        // Stream the whole file (no buffering the entire contents).
        let f2 = fs::File::open(path).with_context(|| format!("re-open {}", path.display()))?;
        let mut r = BufReader::new(f2);

        let mut buf = [0u8; 64 * 1024];
        let mut last_byte: Option<u8> = None;

        loop {
            let n = r.read(&mut buf).with_context(|| format!("read {}", path.display()))?;
            if n == 0 {
                break;
            }
            last_byte = Some(buf[n - 1]);
            w.write_all(&buf[..n])
                .with_context(|| format!("write {}", path.display()))?;
        }

        if last_byte != Some(b'\n') {
            w.write_all(b"\n").context("write trailing newline")?;
        }

        return Ok(());
    }

    let mime = infer
        .get(&sample)
        .map(|t| t.mime_type())
        .unwrap_or("application/octet-stream");

    writeln!(w, "{} (binary, {} bytes)", mime, size).context("write binary description")?;
    Ok(())
}
