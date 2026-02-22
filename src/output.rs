// treecat.rs.d./src/output.rs

use anyhow::{Context, Result};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::thread;

pub struct OrderedWriter {
    tx: Sender<(usize, Vec<u8>)>,
    join: Option<thread::JoinHandle<Result<()>>>,
}

impl OrderedWriter {
    pub fn spawn(bound: usize) -> Result<Self> {
        let (tx, rx) = bounded::<(usize, Vec<u8>)>(bound);
        let join = spawn_ordered_writer(rx);
        Ok(Self {
            tx,
            join: Some(join),
        })
    }

    pub fn send(&self, idx: usize, buf: Vec<u8>) -> Result<()> {
        self.tx.send((idx, buf)).context("send output to writer")?;
        Ok(())
    }

    pub fn finish(mut self) -> Result<()> {
        drop(self.tx);
        if let Some(j) = self.join.take() {
            return j.join().expect("writer thread panicked");
        }
        Ok(())
    }
}

fn spawn_ordered_writer(rx: Receiver<(usize, Vec<u8>)>) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        let mut next = 0usize;
        let mut pending: BTreeMap<usize, Vec<u8>> = BTreeMap::new();

        while let Ok((idx, buf)) = rx.recv() {
            pending.insert(idx, buf);
            while let Some(buf) = pending.remove(&next) {
                stdout.write_all(&buf).context("write output")?;
                next += 1;
            }
        }

        while let Some(buf) = pending.remove(&next) {
            stdout.write_all(&buf).context("write output (drain)")?;
            next += 1;
        }

        stdout.flush().context("flush stdout")?;
        Ok(())
    })
}

