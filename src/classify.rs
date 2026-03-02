// treecat.rs.d/src/classify.rs

pub const SAMPLE_BYTES: usize = 8 * 1024;

pub fn looks_like_text(sample: &[u8]) -> bool {
    if sample.is_empty() {
        return true;
    }

    if sample.contains(&0) {
        return false;
    }

    if std::str::from_utf8(sample).is_ok() {
        return true;
    }

    let mut good = 0usize;
    for &b in sample {
        let ok = matches!(b, b'\n' | b'\r' | b'\t') || (0x20..=0x7E).contains(&b);
        if ok {
            good += 1;
        }
    }

    (good as f64) / (sample.len() as f64) >= 0.90
}
