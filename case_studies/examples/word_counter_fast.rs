// Word count, tuned for top speed. Same job as before — split on whitespace,
// strip ASCII punctuation, lowercase, count, sort by count — but redesigned
// from first principles:
//
//   1. Bytes, not chars: `fs::read` returns raw `Vec<u8>`, skipping UTF-8
//      validation entirely. ASCII handling is byte == byte; multi-byte UTF-8
//      sequences all have the high bit set, so they pass through untouched.
//   2. Lookup tables, not closures: one compile-time 256-entry table answers
//      "is this whitespace?" and another "what does this byte become?"
//      (0 = drop it). Replaces the filter/map/is_ascii_* call chain with a
//      single indexed load per byte.
//   3. Zero allocations in the hot loop: each word is cleaned into one
//      reusable scratch buffer and the map is queried by `&[u8]`. Only the
//      FIRST sighting of a word allocates an owned key — with ~15M tokens but
//      ~1M unique words, 93% of iterations allocate nothing.
//   4. FxHash instead of SipHash: hashing is the single biggest cost in this
//      program; the default hasher is DoS-resistant but ~4x slower.
//   5. Every core: the file is split at whitespace boundaries, each thread
//      counts its chunk into its own map (no locks, no sharing), and the maps
//      are merged once at the end.
//   6. Output: one 1 MiB BufWriter + hand-rolled integer formatting, so a
//      million result lines don't pay the fmt machinery or per-line flushes.
//
// Build/run (release + lto/codegen-units from Cargo.toml matter — a debug
// build of this is slower than the Python version):
//
//     cargo run --release --example temp -- <file>
//
// Behavioral deltas vs the naive version, both deliberate: tokens that clean
// down to nothing ("--", "...") are dropped instead of counted as an empty
// word, and ties are broken alphabetically so output is deterministic.

use rustc_hash::FxHashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::thread;

// ── compile-time byte tables ──────────────────────────────────────────────────

// The five ASCII whitespace bytes: space \t \n \x0c \r.
const IS_WS: [bool; 256] = {
    let mut t = [false; 256];
    t[b' ' as usize] = true;
    t[b'\t' as usize] = true;
    t[b'\n' as usize] = true;
    t[0x0c] = true;
    t[b'\r' as usize] = true;
    t
};

// What each byte becomes inside a word: punctuation → 0 (dropped),
// 'A'..='Z' → lowercase, everything else unchanged. (NUL also maps to 0 and
// gets dropped — irrelevant for text input.)
const XLAT: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut b = 0;
    while b < 256 {
        let c = b as u8;
        t[b] = if c.is_ascii_punctuation() { 0 } else { c.to_ascii_lowercase() };
        b += 1;
    }
    t
};

fn main() -> io::Result<()> {
    // first real argument (args()[0] is the program name itself)
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: temp <file>");
            process::exit(1);
        }
    };

    // whole file as raw bytes — no UTF-8 validation pass
    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error reading {path}: {e}");
            process::exit(1);
        }
    };

    // ── parallel count: one chunk, one thread, one private map ────────────────
    let n_threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let ranges = chunk_ranges(&data, n_threads);

    let maps: Vec<FxHashMap<Box<[u8]>, u32>> = thread::scope(|s| {
        let handles: Vec<_> = ranges
            .iter()
            .map(|&(lo, hi)| {
                let chunk = &data[lo..hi];
                s.spawn(move || count_chunk(chunk))
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    // merge smaller maps into the biggest one (fewest re-inserts)
    let merged = maps
        .into_iter()
        .reduce(|a, b| {
            let (mut big, small) = if a.len() >= b.len() { (a, b) } else { (b, a) };
            for (word, n) in small {
                *big.entry(word).or_insert(0) += n;
            }
            big
        })
        .unwrap_or_default();

    // ── sort by count desc; alphabetical tie-break makes output deterministic ─
    let mut sorted: Vec<(Box<[u8]>, u32)> = merged.into_iter().collect();
    sorted.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // ── write everything through one big buffer ───────────────────────────────
    let mut out = io::BufWriter::with_capacity(1 << 20, io::stdout().lock());
    let mut numbuf = [0u8; 10]; // u32 is at most 10 digits
    for (word, count) in &sorted {
        out.write_all(word)?;
        out.write_all(b" ")?;
        out.write_all(fmt_u32(*count, &mut numbuf))?;
        out.write_all(b"\n")?;
    }
    out.flush()
}

// Split [0, len) into n near-equal ranges, nudging each boundary forward to
// the next whitespace byte so no word straddles two chunks.
fn chunk_ranges(data: &[u8], n: usize) -> Vec<(usize, usize)> {
    let mut bounds = Vec::with_capacity(n + 1);
    bounds.push(0);
    for i in 1..n {
        let mut pos = data.len() * i / n;
        while pos < data.len() && !IS_WS[data[pos] as usize] {
            pos += 1;
        }
        bounds.push(pos);
    }
    bounds.push(data.len());
    bounds.windows(2).map(|w| (w[0], w[1])).collect()
}

// The hot loop. Scans one chunk and returns its private word→count map.
fn count_chunk(chunk: &[u8]) -> FxHashMap<Box<[u8]>, u32> {
    let mut counts: FxHashMap<Box<[u8]>, u32> = FxHashMap::default();
    counts.reserve(1 << 16); // skip the early rehash-and-grow cycles
    let mut scratch: Vec<u8> = Vec::with_capacity(64); // reused for every word
    let mut i = 0;
    while i < chunk.len() {
        // skip the whitespace run
        while i < chunk.len() && IS_WS[chunk[i] as usize] {
            i += 1;
        }
        // clean the word into scratch: one table load per byte
        scratch.clear();
        while i < chunk.len() && !IS_WS[chunk[i] as usize] {
            let t = XLAT[chunk[i] as usize];
            if t != 0 {
                scratch.push(t);
            }
            i += 1;
        }
        if scratch.is_empty() {
            continue; // token was pure punctuation
        }
        // hot path (repeat word): one hash, one lookup, one increment.
        // cold path (first sighting): the only allocation in the loop.
        match counts.get_mut(scratch.as_slice()) {
            Some(n) => *n += 1,
            None => {
                counts.insert(scratch.as_slice().into(), 1);
            }
        }
    }
    counts
}

// u32 → decimal digits, written right-to-left into a stack buffer.
// Avoids the format! machinery (~2x faster for pure integer output).
fn fmt_u32(mut n: u32, buf: &mut [u8; 10]) -> &[u8] {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    &buf[i..]
}
